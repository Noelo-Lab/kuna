#!/usr/bin/env python3
"""Exercise Kuna against a temporary PostgreSQL BSim index."""

import argparse
from collections import Counter
from contextlib import contextmanager
from hashlib import md5
import json
import os
from pathlib import Path
import socket
import subprocess
from tempfile import TemporaryDirectory
from uuid import uuid4
from xml.etree import ElementTree

ROOT = Path(__file__).resolve().parents[1]
ADDRESS = '0x411440'
MD5 = '2527c463a079c81af7b3bc1d26bd3b5d'
REPOSITORY = 'ghidra://fixture.example/BSim/Reference%20Builds/WallaceSrc.exe'
# Ghidra 11.4_DEV DecompInterface.generateSignatures at ADDRESS, settings 0x49.
FEATURES = (
    9273705, 503969765, 861584098, 1212961251, 1415340373, 1415340373,
    1415340373, 1415340373, 1813166669, 1868542086, 1919929135, 2067779848,
    2135386391, 2304150708, 2533570967, 2868931833, 3145426184, 3421229145,
    4138583175, 4193421785,
)
BRANCH_ADDRESS = '0x4011a0'
# Stock Ghidra 12.1.2, settings 0x49, from the matching x86-64 strlen body.
BRANCH_FEATURES = (
    190135962, 507441980, 1209351826, 1283361511, 1391945210, 1799436086,
    1868542086, 1944285574, 1983149460, 2207808987, 2899088587, 3087053545,
    3771742571, 3778997062, 3800300898, 4201356335,
)


def run(*argv, **kwargs):
    return subprocess.run(argv, check=True, capture_output=True, text=True, timeout=45, **kwargs).stdout


def encode_vector(features):
    return '(' + ','.join(f'{n:x}:{h:x}' for h, n in sorted(Counter(features).items())) + ')'


def branch_fixture(root):
    debug, stripped = root/'branch-debug', root/'branch-stripped'
    source = ROOT/'tests/fixtures/fid/bsim_strlen_x86_64.s'
    run('cc', '-g', '-nostdlib', '-static', '-no-pie',
        f'-Wl,-Ttext={BRANCH_ADDRESS},--build-id=none,-e,kuna_strlen', source, '-o', debug)
    run('objcopy', '--strip-all', debug, stripped)
    for binary in (debug, stripped):
        run('objcopy', '--dump-section', f'.text={binary}.text', binary)
    assert Path(f'{debug}.text').read_bytes() == Path(f'{stripped}.text').read_bytes()
    return debug, stripped


def initialize(root, extension, weights, psql):
    sql = (extension / 'lshvector--1.0.sql').read_text()
    sql = '\n'.join(line for line in sql.splitlines() if not line.startswith('\\'))
    library = str(root / 'lshvector').replace("'", "''")
    psql('CREATE TYPE lshvector;\n' + sql.replace('MODULE_PATHNAME', library))
    factory = weights.find('weightfactory')
    values = [float(x.text) for x in factory if x.tag in {'idf', 'tf'}]
    values += [float(factory.findtext(x)) for x in (
        'weightnorm', 'probflip0', 'probflip1', 'probdiff0', 'probdiff1',
    )]
    values += [float(factory.get('scale')), float(factory.get('addend'))]
    assert len(values) == 583
    weight_rows = ','.join(f'({i},{value:.18f})' for i, value in enumerate(values))
    idf_rows = ','.join(f'({int(x.text, 16) & 0xffffffff},{int(x.get("count"))})'
                        for x in weights.find('idflookup'))
    vector = encode_vector(FEATURES)
    branch = encode_vector(BRANCH_FEATURES)
    branch_md5 = md5((root/'branch-debug').read_bytes(), usedforsecurity=False).hexdigest()
    password = uuid4().hex
    psql(f"""
        BEGIN;
        CREATE TABLE keyvaluetable (key text UNIQUE, value text);
        INSERT INTO keyvaluetable VALUES
            ('layout','6'),('major','6'),('minor','1'),('settings','73'),('k','17'),('L','146');
        CREATE TABLE weighttable (id integer, weight numeric(24,20));
        INSERT INTO weighttable VALUES {weight_rows};
        CREATE TABLE idflookup (hash bigint, lookup integer);
        INSERT INTO idflookup VALUES {idf_rows};
        SELECT lsh_load();
        CREATE TABLE vectable (id bigint UNIQUE, count integer, vec lshvector);
        CREATE INDEX ON vectable USING gin (vec gin_lshvector_ops);
        INSERT INTO vectable VALUES (99,2,'{vector}'), (100,1,'{branch}');
        CREATE TABLE exetable (id serial PRIMARY KEY, md5 text UNIQUE, name_exec text,
            architecture integer, name_compiler integer, ingest_date timestamptz, repository integer, path integer);
        INSERT INTO exetable VALUES (1,'{MD5}','WallaceSrc.exe',1,1,now(),1,1);
        INSERT INTO exetable VALUES (2,'{branch_md5}','branch-debug',2,2,now(),NULL,NULL);
        CREATE TABLE desctable (id bigserial PRIMARY KEY, name_func text, id_exe integer,
            id_signature bigint, flags integer, addr bigint);
        CREATE INDEX ON desctable (id_signature);
        INSERT INTO desctable VALUES
            (42,'FUN_00411440',1,99,0,{int(ADDRESS, 16)}),(43,'same_vector',1,99,0,{0x411450});
        INSERT INTO desctable VALUES (44,'kuna_strlen',2,100,0,{int(BRANCH_ADDRESS, 16)});
        CREATE TABLE archtable (id serial PRIMARY KEY, val text UNIQUE);
        INSERT INTO archtable (val) VALUES ('x86:LE:32:default'), ('x86:LE:64:default');
        CREATE TABLE comptable (id serial PRIMARY KEY, val text UNIQUE);
        INSERT INTO comptable (val) VALUES ('windows'), ('gcc');
        CREATE TABLE repotable (id serial PRIMARY KEY, val text UNIQUE);
        INSERT INTO repotable (val) VALUES ('ghidra://fixture.example/BSim');
        CREATE TABLE pathtable (id serial PRIMARY KEY, val text UNIQUE);
        INSERT INTO pathtable (val) VALUES ('Reference Builds');
        CREATE ROLE bsim_reader LOGIN PASSWORD '{password}';
        GRANT SELECT ON ALL TABLES IN SCHEMA public TO bsim_reader;
        COMMIT;
    """)
    return password


@contextmanager
def fixture_db(root, ghidra, headers):
    bsim = ghidra / 'Ghidra/Features/BSim'
    extension = bsim / 'src/lshvector'
    sources = [extension / 'c' / f'{file}.c' for file in ('lsh', 'weights', 'binhash', 'crc32')]
    run('cc', '-shared', '-fPIC', '-I', headers, *sources, '-o', root/'lshvector.so', '-lm')
    run('openssl', 'req', '-x509', '-newkey', 'rsa:2048', '-nodes', '-days', '1',
        '-subj', '/CN=BSim fixture CA', '-addext', 'basicConstraints=critical,CA:TRUE',
        '-keyout', 'ca.key', '-out', 'ca.crt', cwd=root)
    run('openssl', 'req', '-new', '-newkey', 'rsa:2048', '-nodes', '-subj', '/CN=localhost',
        '-addext', 'subjectAltName=DNS:localhost', '-keyout', 'server.key', '-out', 'server.csr', cwd=root)
    run('openssl', 'x509', '-req', '-in', 'server.csr', '-CA', 'ca.crt', '-CAkey', 'ca.key',
        '-CAcreateserial', '-days', '1', '-copy_extensions', 'copy', '-out', 'server.crt', cwd=root)
    with socket.socket() as listener:
        listener.bind(('127.0.0.1', 0))
        port = str(listener.getsockname()[1])
    data = root/'db'
    run('initdb', '-D', data, '-U', 'bsim_admin', '--no-locale', '--encoding=UTF8',
        '--auth-local=trust', '--auth-host=scram-sha-256')
    options = (f'-k {root} -h 127.0.0.1 -p {port} -c ssl=on '
               f'-c ssl_cert_file={root}/server.crt -c ssl_key_file={root}/server.key')
    run('pg_ctl', '-D', data, '-w', '-l', root/'postgres.log', '-o', options, 'start')
    try:
        def psql(sql):
            return run('psql', '-h', root, '-p', port, '-U', 'bsim_admin', '-d', 'postgres',
                       '-X', '-v', 'ON_ERROR_STOP=1', input=sql)

        weights = ElementTree.parse(ROOT/'decompiler/crates/kuna-analysis/src/analyzers/bsim/weights/lshweights_32.xml')
        password = initialize(root, extension, weights, psql)
        yield f'host=localhost hostaddr=127.0.0.1 port={port} dbname=postgres user=bsim_reader password={password}', password, psql
    finally:
        run('pg_ctl', '-D', data, '-w', '-m', 'fast', 'stop')


def check(kuna, binary, branches, env, password, psql):
    def invoke(command, *args, source=binary, address=ADDRESS, timeout=45, **override):
        result = subprocess.run([kuna, 'bsim', command, source, '--address', address, '--json', *args],
                                env={**env, **override}, capture_output=True, text=True, timeout=timeout)
        assert password not in result.stderr and password not in result.stdout
        return result

    query = invoke('query', '--postgres', '--max-matches', '1')
    assert query.returncode == 0, query.stderr
    doc = json.loads(query.stdout)
    match, = doc['matches']
    assert doc['features'] == len(FEATURES) and doc['truncated'], doc
    assert match['similarity'] == 1 and abs(match['significance'] - 28.720363158781602) < 1e-9, match
    assert match['match_id'] == '42' and match['repository_url'] == REPOSITORY, match
    wrong_host = env['KUNA_BSIM_POSTGRES'].replace('host=localhost ', 'host=untrusted.invalid ')
    refused = invoke('query', '--postgres', KUNA_BSIM_POSTGRES=wrong_host)
    assert refused.returncode == 1 and 'TLS trust' in refused.stderr and not refused.stdout, refused.stderr
    print('Native BSim query, vector expansion, and PostgreSQL TLS hostname verification passed')

    with socket.socket() as stalled:
        stalled.bind(('127.0.0.1', 0))
        stalled.listen(1)
        conn = f'host=localhost hostaddr=127.0.0.1 port={stalled.getsockname()[1]} user=bsim_reader'
        refused = invoke('query', '--postgres', timeout=18, KUNA_BSIM_POSTGRES=conn)
        assert refused.returncode == 1 and 'connection startup timed out' in refused.stderr, refused.stderr
        assert not refused.stdout
    print('PostgreSQL TLS startup deadline passed against a silent loopback listener')

    for branch in branches:
        result = run(kuna, 'bsim', 'query', branch, '--postgres', '--address', BRANCH_ADDRESS,
                     '--max-matches', '1', '--json', env=env)
        doc = json.loads(result)
        match_row, = doc['matches']
        assert doc['features'] == len(BRANCH_FEATURES), doc
        assert match_row['name'] == 'kuna_strlen' and abs(match_row['similarity'] - 1.0) < 1e-12, doc
    print('Branch normalization matched stock Ghidra on both debug and stripped ELF')

    def reject_config(change, restore, diagnostic):
        psql(change)
        try:
            result = invoke('query', '--postgres')
            assert result.returncode == 1 and diagnostic in result.stderr and not result.stdout, result.stderr
        finally:
            psql(restore)

    reject_config(
        'CREATE SCHEMA extra; CREATE TABLE extra.weighttable (id integer)',
        'DROP SCHEMA extra CASCADE', 'invalid BSim loader namespace',
    )
    reject_config(
        'ALTER TABLE public.idflookup RENAME TO original_idflookup; '
        'CREATE TABLE public.idflookup AS SELECT * FROM public.original_idflookup LIMIT 1; '
        'GRANT SELECT ON public.idflookup TO bsim_reader',
        'DROP TABLE public.idflookup; ALTER TABLE public.original_idflookup RENAME TO idflookup',
        'invalid BSim IDF lookup table',
    )
    reject_config(
        "UPDATE keyvaluetable SET value = '0' WHERE key = 'k'",
        "UPDATE keyvaluetable SET value = '17' WHERE key = 'k'",
        'invalid BSim LSH configuration',
    )
    print('Invalid PostgreSQL BSim loader configuration rejected')

    debug, stripped = branches
    def compare(reference, source=stripped, address=BRANCH_ADDRESS):
        return invoke('compare', '--match-id', '44', '--reference', reference,
                      source=source, address=address)

    good = compare(debug)
    assert good.returncode == 0, good.stderr
    doc = json.loads(good.stdout)
    assert doc['reference']['identity_verified'] and doc['diff'] is not None, doc
    assert doc['match']['match_id'] == '44', doc

    wrong = compare(stripped)
    assert wrong.returncode == 1 and 'MD5' in wrong.stderr and not wrong.stdout, wrong.stderr

    imports = ROOT/'decompiler/crates/kuna-analysis/tests/fixtures/pe_iatincode_i386.exe'
    unavailable = compare(debug, source=imports, address='0x401000')
    assert unavailable.returncode == 0, unavailable.stderr
    doc = json.loads(unavailable.stdout)
    assert doc['source']['address'] == '0x401000', doc
    code = doc['source']['code']
    assert code['text'] is None and not code['complete'] and 'IAT slot' in code['error'], doc
    assert doc['diff'] is None and doc['reference']['code']['complete'], doc
    print('Local matched binary comparison, MD5 verification, and incomplete-code handling passed')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('ghidra_source', type=Path, help='Ghidra source checkout with the WallaceSrc fixture')
    parser.add_argument('--kuna', type=Path, default=ROOT/'decompiler/target/release/kuna')
    parser.add_argument('--pg-headers', type=Path, help='PostgreSQL server headers; defaults to pg_config --includedir-server')
    args = parser.parse_args()
    headers = args.pg_headers or Path(run('pg_config', '--includedir-server').strip())
    binary = args.ghidra_source.resolve()/'GhidraDocs/GhidraClass/ExerciseFiles/VersionTracking/WallaceSrc.exe'
    assert md5(binary.read_bytes(), usedforsecurity=False).hexdigest() == MD5, 'unexpected WallaceSrc fixture'
    with TemporaryDirectory(prefix='kuna-bsim-') as directory:
        root = Path(directory)
        branches = branch_fixture(root)
        with fixture_db(root, args.ghidra_source.resolve(), headers) as (conninfo, password, psql):
            check(args.kuna.resolve(), binary, branches, {**os.environ, 'SLEIGHHOME': str(ROOT/'specs'),
                  'SSL_CERT_FILE': str(root/'ca.crt'), 'KUNA_BSIM_POSTGRES': conninfo}, password, psql)


if __name__ == '__main__':
    try:
        main()
    except subprocess.CalledProcessError as error:
        raise SystemExit(f'{Path(error.cmd[0]).name} failed: {error.stderr}') from None
