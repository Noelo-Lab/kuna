#!/usr/bin/env python3
"""Exercise Ghidra migration, native import, and publishing with the built client and server."""

import argparse
from contextlib import contextmanager
import hashlib
import json
import os
from pathlib import Path
from queue import Queue
from shutil import copy2
import stat
import subprocess
import tempfile
from threading import Thread
from zipfile import ZipFile

ROOT = Path(__file__).resolve().parents[1]
ARCHIVE = ROOT / 'tests/fixtures/bsim/reference.gzf'
UNNAMED_ARCHIVE = ROOT / 'tests/fixtures/bsim/unnamed.gzf'
PROJECT = ROOT / 'tests/fixtures/bsim/stock-project.zip'
RELOCATABLE_ARCHIVE = ROOT / 'tests/fixtures/bsim/relocatable.gzf'
PATCHED_RELOCATABLE_ARCHIVE = ROOT / 'tests/fixtures/bsim/relocatable-patched.gzf'
RELOCATABLE_OBJECT = ROOT / 'tests/fixtures/bsim/relocatable.o'
ARM_RELOCATABLE_ARCHIVE = ROOT / 'tests/fixtures/bsim/relocatable-arm.gzf'
ARM_RELOCATABLE_OBJECT = ROOT / 'decompiler/crates/kuna-analysis/tests/fixtures/arm_thumb_le32.o'
ARM_RELOCATIONS_OBJECT = ROOT / 'tests/fixtures/bsim/arm-relocations.o'
ARM_RELOCATIONS_ARCHIVE = ROOT / 'tests/fixtures/bsim/arm-relocations.gzf'
PATCHED_ARM_RELOCATIONS_ARCHIVE = ROOT / 'tests/fixtures/bsim/arm-relocations-patched.gzf'
ORIGINAL = ROOT / 'tests/fixtures/fid/callees/prog'
OBJECT = ROOT / 'decompiler/crates/kuna-analysis/tests/fixtures/fid_lib_x86_64.o'
UNIVERSAL = ROOT / 'decompiler/crates/kuna-analysis/tests/fixtures/macho_fat'


def command(env, *args, status=0):
    result = subprocess.run(args, env=env, capture_output=True, text=True, timeout=60)
    if result.returncode != status:
        raise AssertionError(f'{args!r}: exit {result.returncode}\n{result.stdout}{result.stderr}')
    return result


@contextmanager
def server_url(server, store, env):
    args = [server, '--store', store, '--bind', '127.0.0.1:0']
    with subprocess.Popen(args, env=env, text=True, stderr=subprocess.PIPE) as child:
        try:
            ready = Queue()
            reader = Thread(target=lambda: ready.put(child.stderr.readline()), daemon=True)
            reader.start()
            line = ready.get(timeout=10)
            prefix = 'kuna-bsim-server listening on http://127.0.0.1:'
            if not line.startswith(prefix):
                raise AssertionError(f'expected the server to listen: {line!r}')
            yield line.strip().rsplit(' ', 1)[1]
        finally:
            if child.poll() is None:
                child.terminate()
                try:
                    child.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    child.kill()
                    child.wait()


def isolated_env(scratch, embedded):
    env = {key: value for key, value in os.environ.items()
           if not key.startswith('KUNA_') and key not in ('SLEIGHHOME', 'kuna_fid_db')}
    env['PATH'] = str(scratch)
    if embedded:
        cache = scratch / 'runtime'
        cache.mkdir()
        env.update({key: str(cache) for key in
                    ('HOME', 'LOCALAPPDATA', 'XDG_CACHE_HOME', 'TMPDIR', 'TEMP', 'TMP')})
    else:
        env.update(KUNA_SPECS=str(ROOT / 'specs'), SLEIGHHOME=str(ROOT / 'specs'))
    return env


def check_archive(kuna, server, scratch, env):
    store = scratch / 'store'
    sha256 = hashlib.sha256(ORIGINAL.read_bytes()).hexdigest()
    report = json.loads(command(env, server, 'import-ghidra', ARCHIVE, '--store', store,
                                '--json').stdout)
    assert report['imported'] == 1 and report['failed'] == report['incomplete'] == 0, report
    (item,) = report['items']
    assert item['sha256'] == sha256 and item['snapshot_complete'] and not item['executable_complete'], item

    text = command(env, server, 'import-ghidra', ARCHIVE, '--store', store)
    assert f"{item['signed']}/{item['considered']} named function(s) signed" in text.stdout, text.stdout
    assert f"{item['skipped_unnamed']} unnamed skipped" in text.stdout, text.stdout
    assert 'native coverage' not in text.stdout and not text.stderr, text

    skipped_store = scratch / 'unnamed-store'
    skipped = json.loads(command(env, server, 'import-ghidra', UNNAMED_ARCHIVE,
                                 '--store', skipped_store, '--json').stdout)
    assert skipped['skipped'] == 1 and skipped['imported'] == skipped['failed'] == 0, skipped
    assert skipped['items'][0]['status'] == 'skipped' and skipped['items'][0]['skipped_unnamed'] == 3, skipped
    assert not (skipped_store / 'binaries' / sha256).exists(), skipped

    with server_url(server, store, env) as origin:
        query = json.loads(command(env, kuna, 'bsim', 'query', ORIGINAL, '--address', '0x401000',
                                   '--server', origin, '--json').stdout)
        hit = query['matches'][0]
        assert hit['name'] == 'project_scale_a' and hit['address'] == '0x401000', query
        assert hit['sha256'] == sha256 and abs(hit['similarity'] - 1.0) < 1e-9, query
        assert not hit['complete'], query

        args = [kuna, 'bsim', 'compare', ORIGINAL, '--address', '0x401020', '--server', origin,
                '--match-id', hit['match_id'], '--json']
        comparison = json.loads(command(env, *args).stdout)
        reference = comparison['reference']
        assert reference['identity_verified'] and reference['sha256'] == sha256, comparison
        assert comparison['match']['name'] == 'project_scale_a', comparison
        assert all(comparison[side]['code']['complete'] for side in ('source', 'reference')), comparison
        diff = comparison['diff'].splitlines()
        assert any(line.startswith('- ') and 'return' in line for line in diff), comparison
        assert any(line.startswith('+ ') and 'return' in line for line in diff), comparison

        # A same-length replacement cannot pass the client's identity check.
        blob = store / 'binaries' / sha256
        contents = bytearray(blob.read_bytes())
        contents[-1] ^= 0x80
        blob.write_bytes(contents)
        rejected = command(env, *args, status=1)
        assert not rejected.stdout and 'SHA-256 mismatch' in rejected.stderr, rejected


@contextmanager
def read_only_source(root):
    modes = {}
    try:
        if os.name == 'posix':
            for path in [root, *root.rglob('*')]:
                modes[path] = stat.S_IMODE(path.stat().st_mode)
                path.chmod(modes[path] & ~0o222)
        yield
    finally:
        for path, mode in modes.items():
            if path.exists():
                path.chmod(mode)


def snapshot(root):
    entries = {}
    for path in [root, *root.rglob('*')]:
        metadata = path.stat()
        digest = hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None
        entries[path.relative_to(root)] = (metadata.st_mode, metadata.st_mtime_ns, digest)
    return entries


def original_match(kuna, origin, env, digest, address):
    query = json.loads(command(env, kuna, 'bsim', 'query', ORIGINAL, '--address', address,
                               '--server', origin, '--json').stdout)
    hits = [hit for hit in query['matches'] if hit['sha256'] == digest and hit['address'] == address]
    assert len(hits) == 1, query
    hit = hits[0]
    assert hit['complete'] and abs(hit['similarity'] - 1.0) < 1e-9, query
    return hit


def check_project_migration(kuna, server, scratch, store, env, digest, published_id):
    addresses = ('0x401000', '0x401020', '0x401040', '0x401090')
    with server_url(server, store, env) as origin:
        native = {addr: original_match(kuna, origin, env, digest, addr) for addr in addresses}
    assert native['0x401000']['match_id'] == published_id, native
    assert native['0x401000']['name'] == 'reviewed_scale', native
    assert native['0x401020']['name'] != 'project_scale_b', native
    assert all(hit['executable'] == ORIGINAL.name for hit in native.values()), native

    source = scratch / 'stock-project'
    with ZipFile(PROJECT) as fixture:
        fixture.extractall(source)
    with read_only_source(source):
        before = snapshot(source)
        for suffix, outcome, updated, unchanged in [('gpr', 'updated', 2, 0), ('rep', 'already_present', 0, 2)]:
            path = source / f'bsim.{suffix}'
            report = json.loads(command(env, server, 'import-ghidra', path, '--store', store,
                                        '--json').stdout)
            assert report['imported'] == 1 and report['failed'] == report['incomplete'] == 0, report
            (item,) = report['items']
            assert item['item'] == '/annotated-prog' and item['display_name'] == 'annotated-prog', item
            assert item['version'] == 1 and item['sha256'] == digest, item
            assert item['outcome'] == outcome and item['signed'] == item['considered'] == 3, item
            assert item['snapshot_complete'] and item['executable_complete'], item
            assert item['updated'] == updated and item['unchanged'] == unchanged, item
            assert item['inserted'] == 0 and item['kept_published'] == 1, item
            assert snapshot(source) == before, f'{path} source snapshot changed'

    # A new server must see both saved names and the explicit name. The native
    # entry absent from the Ghidra snapshot must remain unchanged.
    expected = native | {
        '0x401020': native['0x401020'] | {'name': 'project_scale_b'},
        '0x401090': native['0x401090'] | {'name': 'entry'},
    }
    with server_url(server, store, env) as origin:
        persisted = {addr: original_match(kuna, origin, env, digest, addr) for addr in addresses}
    assert persisted == expected, persisted


def check_relocatable_migration(kuna, server, scratch, env):
    store = scratch / 'relocatable-store'
    original = RELOCATABLE_OBJECT.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    report = json.loads(command(env, server, 'import-ghidra', RELOCATABLE_ARCHIVE,
                                '--store', store, '--json').stdout)
    assert report['imported'] == 1 and report['failed'] == report['incomplete'] == 0, report
    (item,) = report['items']
    assert item['snapshot_complete'] and not item['executable_complete'], item
    assert item['considered'] == item['signed'] == 2 and item['sha256'] == digest, item
    assert (store / 'binaries' / digest).read_bytes() == original

    rejected_store = scratch / 'patched-relocatable-store'
    rejected = json.loads(command(env, server, 'import-ghidra', PATCHED_RELOCATABLE_ARCHIVE,
                                  '--store', rejected_store, '--json', status=1).stdout)
    assert rejected['failed'] == 1 and rejected['imported'] == rejected['incomplete'] == 0, rejected
    assert 'disagrees with the verified stock ELF relocation' in rejected['items'][0]['error'], rejected
    assert not (rejected_store / 'binaries' / digest).exists(), rejected

    with server_url(server, store, env) as origin:
        hits = []
        for suffix in ('a', 'b'):
            query = json.loads(command(env, kuna, 'bsim', 'query', RELOCATABLE_OBJECT,
                                       '--name', f'original_rel_{suffix}', '--server', origin,
                                       '--json').stdout)
            (hit,) = [row for row in query['matches'] if row['sha256'] == digest
                      and row['address'] == query['function']['address']]
            assert hit['name'] == f'project_rel_{suffix}', hit
            assert abs(hit['similarity'] - 1.0) < 1e-9, hit
            hits.append(hit)

        assert hits[0]['address'] != hits[1]['address'], hits
        comparison = json.loads(command(
            env, kuna, 'bsim', 'compare', RELOCATABLE_OBJECT, '--name', 'original_rel_a',
            '--server', origin, '--match-id', hits[1]['match_id'], '--json',
        ).stdout)
        reference = comparison['reference']
        assert reference['identity_verified'] and reference['sha256'] == digest, comparison
        assert reference['address'] == hits[1]['address'], comparison
        assert comparison['match']['name'] == 'project_rel_b', comparison
        assert all(comparison[side]['code']['complete'] for side in ('source', 'reference')), comparison
        assert comparison['diff'], comparison


def check_thumb_relocation(kuna, server, scratch, env):
    store = scratch / 'thumb-relocatable-store'
    original = ARM_RELOCATABLE_OBJECT.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    report = json.loads(command(env, server, 'import-ghidra', ARM_RELOCATABLE_ARCHIVE,
                                '--store', store, '--json').stdout)
    assert report['imported'] == 1 and report['failed'] == report['incomplete'] == 0, report
    assert (store / 'binaries' / digest).read_bytes() == original
    with server_url(server, store, env) as origin:
        query = json.loads(command(env, kuna, 'bsim', 'query', ARM_RELOCATABLE_OBJECT,
                                   '--name', 'thumb_add', '--server', origin, '--json').stdout)
        (hit,) = [row for row in query['matches'] if row['sha256'] == digest
                  and row['address'] == query['function']['address']]
        assert hit['name'] == 'project_thumb_add' and abs(hit['similarity'] - 1.0) < 1e-9, hit


def check_stock_arm_relocations(kuna, server, scratch, env):
    store = scratch / 'stock-arm-relocations'
    original = ARM_RELOCATIONS_OBJECT.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    report = json.loads(command(env, server, 'import-ghidra', ARM_RELOCATIONS_ARCHIVE,
                                '--store', store, '--json').stdout)
    assert report['imported'] == 1 and report['failed'] == report['incomplete'] == 0, report
    (item,) = report['items']
    assert item['sha256'] == digest and item['signed'] == item['considered'] > 0, item
    assert (store / 'binaries' / digest).read_bytes() == original

    with server_url(server, store, env) as origin:
        query = json.loads(command(env, kuna, 'bsim', 'query', ARM_RELOCATIONS_OBJECT,
                                   '--name', 'fixture_arm_relocations', '--server', origin,
                                   '--json').stdout)
        (hit,) = [row for row in query['matches'] if row['sha256'] == digest
                  and row['address'] == query['function']['address']]
        assert hit['name'] == 'project_arm_relocations' and abs(hit['similarity'] - 1.0) < 1e-9, hit

    rejected_store = scratch / 'patched-stock-arm-relocations'
    rejected = json.loads(command(env, server, 'import-ghidra', PATCHED_ARM_RELOCATIONS_ARCHIVE,
                                  '--store', rejected_store, '--json', status=1).stdout)
    assert rejected['failed'] == 1 and rejected['imported'] == rejected['incomplete'] == 0, rejected
    assert 'disagrees with the verified stock ELF relocation' in rejected['items'][0]['error'], rejected
    assert not (rejected_store / 'binaries' / digest).exists(), rejected


def check_publication(kuna, server, scratch, env):
    env = env | {
        'KUNA_BSIM_SERVER_TOKEN': 'fixture-reader',
        'KUNA_BSIM_SERVER_WRITE_TOKEN': 'fixture-publisher',
    }
    store = scratch / 'native-store'
    command(env, server, 'import', OBJECT, '--store', store, '--json')
    digest = hashlib.sha256(OBJECT.read_bytes()).hexdigest()

    with server_url(server, store, env) as origin:
        def match(source_name):
            query = json.loads(command(env, kuna, 'bsim', 'query', OBJECT, '--name', source_name,
                                       '--server', origin, '--json').stdout)
            hit = query['matches'][0]
            assert hit['sha256'] == digest and hit['address'] == query['function']['address'], query
            assert hit['complete'] and abs(hit['similarity'] - 1.0) < 1e-9, query
            return hit

        initial = match('kuna_crc32')
        assert initial['name'] == 'kuna_crc32', initial
        assert initial['similarity'] == 1.0, initial
        publish = [kuna, 'bsim', 'push', OBJECT, '--server', origin, '--name', 'kuna_crc32',
                   '--rename', 'reviewed_crc32', '--json']
        read_only = env | {'KUNA_BSIM_SERVER_WRITE_TOKEN': env['KUNA_BSIM_SERVER_TOKEN']}
        rejected = command(read_only, *publish, status=1)
        assert not rejected.stdout and '401' in rejected.stderr, rejected
        assert match('kuna_crc32')['name'] == 'kuna_crc32'

        pushed = json.loads(command(env, *publish).stdout)
        assert pushed['match_id'] == initial['match_id'] and pushed['outcome'] == 'updated', pushed
        assert pushed['name'] == 'reviewed_crc32' and pushed['sha256'] == digest, pushed
        again = json.loads(command(env, *publish).stdout)
        assert again['outcome'] == 'already_present', again
        assert match('kuna_crc32')['name'] == 'reviewed_crc32'
        assert match('kuna_memset')['name'] == 'kuna_memset'

        result = json.loads(command(env, kuna, 'bsim', 'compare', OBJECT, '--name', 'kuna_crc32',
                                    '--server', origin, '--match-id', pushed['match_id'], '--json').stdout)
        assert result['match']['name'] == 'reviewed_crc32'
        assert result['reference']['sha256'] == digest and result['reference']['code']['complete'], result

        first = json.loads(command(env, kuna, 'bsim', 'push', ORIGINAL, '--server', origin,
                                   '--address', '0x401000', '--rename', 'reviewed_scale', '--json').stdout)
        original_digest = hashlib.sha256(ORIGINAL.read_bytes()).hexdigest()
        assert first['outcome'] == 'inserted' and first['sha256'] == original_digest, first

    report = json.loads(command(env, server, 'import', ORIGINAL, OBJECT, '--store', store,
                                '--json').stdout)
    assert report['imported'] == 2 and report['incomplete'] == report['failed'] == 0, report
    check_project_migration(kuna, server, scratch, store, env, original_digest, first['match_id'])


def check_universal(kuna, server, scratch, env):
    original = UNIVERSAL.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    for slice_name, architecture in [(None, 'x86:LE:64:'), ('arm64', 'AARCH64:LE:64:')]:
        indexing_env = env if slice_name is None else env | {'KUNA_MACHO_SLICE': slice_name}
        store = scratch / f'macho-{slice_name or "default"}'
        report = json.loads(command(indexing_env, server, 'import', UNIVERSAL, '--store', store,
                                    '--json').stdout)
        (item,) = report['items']
        assert report['failed'] == report['incomplete'] == 0 and item['complete'], report
        assert item['sha256'] == digest and (store / 'binaries' / digest).read_bytes() == original
        if slice_name is None:
            alternate_env = env | {'KUNA_MACHO_SLICE': 'arm64'}
            conflict = json.loads(command(alternate_env, server, 'import', UNIVERSAL,
                                          '--store', store, '--json', status=1).stdout)
            assert conflict['failed'] == 1 and conflict['imported'] == 0, conflict
        with server_url(server, store, env) as origin:
            query = json.loads(command(indexing_env, kuna, 'bsim', 'query', UNIVERSAL,
                                       '--name', '_main', '--server', origin, '--json').stdout)
            (hit,) = [hit for hit in query['matches'] if hit['sha256'] == digest
                      and hit['address'] == query['function']['address']]
            assert hit['architecture'].startswith(architecture) and hit['name'] == '_main', hit
            assert hit['macho_slice'] == item['macho_slice'], hit
            assert abs(hit['similarity'] - 1.0) < 1e-9, hit

            # The client source uses the default slice, even for an arm64
            # reference. Its local settings must not select the reference.
            comparison = json.loads(command(env, kuna, 'bsim', 'compare', UNIVERSAL,
                                            '--name', '_main', '--server', origin,
                                            '--match-id', hit['match_id'], '--json').stdout)
            reference = comparison['reference']
            assert reference['identity_verified'] and reference['sha256'] == digest, comparison
            assert comparison['match']['macho_slice'] == hit['macho_slice'], comparison
            assert reference['address'] == hit['address'] and reference['code']['complete'], comparison
            assert 'printf(' in reference['code']['text'], comparison


def check_embedded_server_specs(kuna, server, scratch, env):
    runtime = scratch / 'server-runtime'
    runtime.mkdir()
    store = scratch / 'embedded-store'
    server_env = env | {key: str(runtime) for key in
                        ('HOME', 'LOCALAPPDATA', 'XDG_CACHE_HOME', 'TMPDIR', 'TEMP', 'TMP')}
    server_env['KUNA_BSIM_SERVER_WRITE_TOKEN'] = 'fixture-embedded-writer'
    command(server_env, server, 'import', OBJECT, '--store', store, '--json')
    with server_url(server, store, server_env) as origin:
        client_env = env | {'KUNA_BSIM_SERVER_WRITE_TOKEN': server_env['KUNA_BSIM_SERVER_WRITE_TOKEN']}
        result = json.loads(command(client_env, kuna, 'bsim', 'push', OBJECT, '--server', origin,
                                    '--name', 'kuna_crc32', '--rename', 'embedded_crc32', '--json').stdout)
        assert result['outcome'] == 'updated' and result['name'] == 'embedded_crc32', result
    assert not list(runtime.iterdir()), 'embedded server unpacked SLEIGH files or left staging files'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--kuna', type=Path, required=True)
    parser.add_argument('--server', type=Path, required=True)
    parser.add_argument('--embedded', action='store_true',
                        help='copy release binaries outside the checkout and use only bundled specs and weights')
    args = parser.parse_args()
    kuna, server = args.kuna.resolve(), args.server.resolve()
    with tempfile.TemporaryDirectory(prefix='kuna-bsim-test-') as directory:
        scratch = Path(directory).resolve()
        env = isolated_env(scratch, args.embedded)
        if args.embedded:
            if scratch.is_relative_to(ROOT):
                raise AssertionError('embedded smoke test requires a temporary directory outside the checkout')
            kuna = Path(copy2(kuna, scratch))
            server = Path(copy2(server, scratch))
        previous = Path.cwd()
        os.chdir(scratch)
        try:
            check_archive(kuna, server, scratch, env)
            check_relocatable_migration(kuna, server, scratch, env)
            check_thumb_relocation(kuna, server, scratch, env)
            check_stock_arm_relocations(kuna, server, scratch, env)
            check_publication(kuna, server, scratch, env)
            check_universal(kuna, server, scratch, env)
            if args.embedded:
                check_embedded_server_specs(kuna, server, scratch, env)
        finally:
            os.chdir(previous)
    print('Ghidra linked and relocatable migration, native and universal Mach-O import, publish, C diff, and tamper rejection passed')


if __name__ == '__main__':
    main()
