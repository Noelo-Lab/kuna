## Headline

| scope | n | changed | GED base | GED var | perfect base | perfect var | dPerfect | dGED |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| O0 | 32339 | 3014 | 230291 | 227475 | 14785 | 15396 | +611 | -2816 |
| O2 | 22546 | 1740 | 396062 | 397294 | 7422 | 7409 | -13 | +1232 |
| O2-noinline | 30310 | 2165 | 286500 | 287117 | 11129 | 11114 | -15 | +617 |
| ALL | 85195 | 6919 | 912853 | 911886 | 33336 | 33919 | +583 | -967 |

## Both directions

- to perfect **781**, off perfect **198** (ratio 3.94 : 1, McNemar z = +18.63)
- improved 3254 / worsened 2882
- hybrids 312, unresolved 0

  - O0: to perfect 663, off perfect 52, improved 1615, worsened 1052
  - O2: to perfect 41, off perfect 54, improved 639, worsened 910
  - O2-noinline: to perfect 77, off perfect 92, improved 1000, worsened 920

### per opt level

| cell | n | changed | dGED | to perfect | off perfect | net |
|---|---:|---:|---:|---:|---:|---:|
| O2-noinline | 30310 | 2165 | +617 | 77 | 92 | -15 |
| O2 | 22546 | 1740 | +1232 | 41 | 54 | -13 |
| O0 | 32339 | 3014 | -2816 | 663 | 52 | +611 |

### per architecture

| cell | n | changed | dGED | to perfect | off perfect | net |
|---|---:|---:|---:|---:|---:|---:|
| pe-x86 | 460 | 44 | +11 | 8 | 2 | +6 |
| arm | 31058 | 1154 | +320 | 75 | 28 | +47 |
| x86-64 | 53677 | 5721 | -1298 | 698 | 168 | +530 |

### per project

| cell | n | changed | dGED | to perfect | off perfect | net |
|---|---:|---:|---:|---:|---:|---:|
| bzip2 | 281 | 34 | +44 | 1 | 4 | -3 |
| libselinux | 1136 | 72 | +49 | 9 | 12 | -3 |
| kmod | 283 | 11 | +14 | 0 | 2 | -2 |
| libexpat | 158 | 2 | -2 | 0 | 0 | +0 |
| minipig | 9 | 0 | +0 | 0 | 0 | +0 |
| riot-os | 168 | 2 | +0 | 0 | 0 | +0 |
| sysvinit | 499 | 41 | -40 | 1 | 1 | +0 |
| x0r-usb | 55 | 4 | +5 | 0 | 0 | +0 |
| chibios | 995 | 39 | +292 | 1 | 0 | +1 |
| gnutls | 2303 | 231 | +308 | 17 | 16 | +1 |
| grep | 258 | 19 | -34 | 1 | 0 | +1 |
| mirai | 176 | 20 | +9 | 1 | 0 | +1 |
| mydoom | 226 | 26 | +4 | 3 | 2 | +1 |
| base-passwd | 119 | 5 | +2 | 2 | 0 | +2 |
| dpkg | 839 | 46 | +107 | 6 | 4 | +2 |
| findutils | 806 | 54 | -55 | 2 | 0 | +2 |
| iproute2 | 1794 | 160 | +277 | 8 | 6 | +2 |
| libbsd | 153 | 8 | -16 | 2 | 0 | +2 |
| shadow | 734 | 64 | +7 | 5 | 3 | +2 |
| libopencm3 | 2523 | 30 | +6 | 3 | 0 | +3 |
| nuttx | 960 | 69 | -71 | 4 | 1 | +3 |
| rsyslog | 354 | 31 | -51 | 3 | 0 | +3 |
| gzip | 401 | 20 | -54 | 4 | 0 | +4 |
| dexter | 170 | 14 | +2 | 5 | 0 | +5 |
| diffutils | 433 | 30 | -55 | 5 | 0 | +5 |
| libedit | 1396 | 120 | +130 | 13 | 7 | +6 |
| e2fsprogs | 962 | 126 | +135 | 19 | 12 | +7 |
| tar | 1636 | 149 | -127 | 10 | 3 | +7 |
| cleanflight | 5618 | 191 | -38 | 15 | 6 | +9 |
| libacl | 395 | 37 | +37 | 9 | 0 | +9 |
| cronie | 540 | 51 | +5 | 11 | 0 | +11 |
| betaflight | 10406 | 383 | +109 | 24 | 9 | +15 |
| crazyflie | 10388 | 440 | +22 | 28 | 12 | +16 |
| coreutils | 4951 | 331 | -191 | 25 | 2 | +23 |
| zlib | 2104 | 555 | -139 | 71 | 20 | +51 |
| bash | 6566 | 749 | -1138 | 83 | 5 | +78 |
| openssh-portable | 24400 | 2755 | -520 | 390 | 71 | +319 |

### Biggest single movements

| direction | function | base | var |
|---|---|---:|---:|
| to perfect | O0 iproute2/ip::sci_complete | 21 | 0 |
| to perfect | O0 gzip/gzip::make_ofname | 20 | 0 |
| to perfect | O0 bash/bash::skipname | 17 | 0 |
| to perfect | O0 bash/bash::wskipname | 17 | 0 |
| to perfect | O0 bash/bash::sv_bell_style | 15 | 0 |
| to perfect | O0 crazyflie/cf2::sensorsFindBiasValue | 15 | 0 |
| to perfect | O0 crazyflie/firmware::sensorsFindBiasValue | 15 | 0 |
| to perfect | O0 betaflight/betaflight_STM32F405::nextArg | 14 | 0 |
| to perfect | O0 cleanflight/cleanflight_DALRCF405::nextArg | 14 | 0 |
| to perfect | O2 betaflight/betaflight_STM32F405::nextArg | 14 | 0 |
| to perfect | O2 cleanflight/cleanflight_DALRCF405::nextArg | 14 | 0 |
| to perfect | O2-noinline betaflight/betaflight_STM32F405::nextArg | 14 | 0 |
| off perfect | O2-noinline iproute2/ip::print_tuntap | 0 | 19 |
| off perfect | O2-noinline e2fsprogs/e2fsck::write_directory | 0 | 14 |
| off perfect | O0 e2fsprogs/e2fsck::expand_dirent_expression | 0 | 13 |
| off perfect | O0 shadow/lastlog::print_one | 0 | 13 |
| off perfect | O0 coreutils/factor::factor | 0 | 12 |
| off perfect | O0 bash/bash::sv_xtracefd | 0 | 11 |
| off perfect | O0 e2fsprogs/e2fsck::e2fsck_simple_progress | 0 | 11 |
| off perfect | O0 openssh-portable/ssh-keygen::write_checkpoint | 0 | 11 |
| off perfect | O0 shadow/faillog::print_one | 0 | 11 |
| off perfect | O2-noinline e2fsprogs/e2fsck::e2fsck_simple_progress | 0 | 11 |
| off perfect | O0 openssh-portable/scp::sshbuf_free | 0 | 10 |
| off perfect | O0 openssh-portable/sftp-server::sshbuf_free | 0 | 10 |
