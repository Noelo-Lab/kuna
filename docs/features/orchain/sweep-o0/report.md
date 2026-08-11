# optsweep: `--option orchain on`

kuna `f3104ccd` -- baseline arm = shipped defaults, variant arm = the flip. 265 slices, 32339 functions scored in both arms.

## Controls

- unchanged bodies: **29325**; scoring differently: **0** (OK)
- slug-only diffs: **30**; scoring differently: **0** (OK)
- baseline arm agrees with the tree's published verdict on **32263/32339** (99.8%)

## Headline

| scope | n | changed | GED base | GED var | perfect base | perfect var | dPerfect | dGED |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| O0 | 32339 | 3014 | 230291 | 227475 | 14785 | 15396 | +611 | -2816 |
| ALL | 32339 | 3014 | 230291 | 227475 | 14785 | 15396 | +611 | -2816 |

## Both directions

- **to perfect: 663**, **off perfect: 52** (ratio 12.75 : 1, McNemar z = +22.85)
- improved 1615 / worsened 1052

### per opt level

| cell | n | dGED | to perfect | off perfect | net |
|---|---:|---:|---:|---:|---:|
| O0 | 32339 | -2816 | 663 | 52 | +611 |

### per architecture

| cell | n | dGED | to perfect | off perfect | net |
|---|---:|---:|---:|---:|---:|
| pe-x86 | 191 | -5 | 6 | 1 | +5 |
| arm | 11440 | -30 | 41 | 7 | +34 |
| x86-64 | 20708 | -2781 | 616 | 44 | +572 |

### per project

| cell | n | dGED | to perfect | off perfect | net |
|---|---:|---:|---:|---:|---:|
| chibios | 410 | +32 | 0 | 0 | +0 |
| kmod | 129 | -1 | 0 | 0 | +0 |
| libexpat | 58 | +0 | 0 | 0 | +0 |
| minipig | 3 | +0 | 0 | 0 | +0 |
| riot-os | 49 | +0 | 0 | 0 | +0 |
| sysvinit | 195 | -41 | 1 | 1 | +0 |
| x0r-usb | 29 | +1 | 0 | 0 | +0 |
| base-passwd | 48 | -5 | 1 | 0 | +1 |
| bzip2 | 120 | -34 | 1 | 0 | +1 |
| dpkg | 325 | +64 | 4 | 3 | +1 |
| grep | 105 | -21 | 1 | 0 | +1 |
| mirai | 67 | +8 | 1 | 0 | +1 |
| findutils | 313 | -38 | 2 | 0 | +2 |
| gzip | 150 | -33 | 2 | 0 | +2 |
| libbsd | 64 | -27 | 2 | 0 | +2 |
| mydoom | 98 | +2 | 3 | 1 | +2 |
| shadow | 312 | +8 | 5 | 3 | +2 |
| cleanflight | 1862 | -11 | 5 | 2 | +3 |
| dexter | 61 | -8 | 3 | 0 | +3 |
| libopencm3 | 1013 | -28 | 3 | 0 | +3 |
| nuttx | 397 | -20 | 3 | 0 | +3 |
| rsyslog | 139 | -32 | 3 | 0 | +3 |
| diffutils | 172 | -14 | 4 | 0 | +4 |
| betaflight | 3451 | +41 | 8 | 3 | +5 |
| iproute2 | 762 | +120 | 6 | 1 | +5 |
| libacl | 144 | -7 | 5 | 0 | +5 |
| cronie | 199 | +10 | 6 | 0 | +6 |
| libselinux | 416 | -7 | 7 | 0 | +7 |
| tar | 651 | -83 | 7 | 0 | +7 |
| e2fsprogs | 388 | +61 | 16 | 7 | +9 |
| gnutls | 870 | +61 | 15 | 6 | +9 |
| libedit | 508 | +56 | 11 | 2 | +9 |
| crazyflie | 4258 | -44 | 22 | 2 | +20 |
| coreutils | 2054 | -252 | 24 | 1 | +23 |
| bash | 2572 | -621 | 60 | 1 | +59 |
| zlib | 774 | -416 | 66 | 5 | +61 |
| openssh-portable | 9173 | -1537 | 366 | 14 | +352 |

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
| to perfect | O0 bash/bash::maybe_call_trap_handler | 13 | 0 |
| off perfect | O0 e2fsprogs/e2fsck::expand_dirent_expression | 0 | 13 |
| off perfect | O0 shadow/lastlog::print_one | 0 | 13 |
| off perfect | O0 coreutils/factor::factor | 0 | 12 |
| off perfect | O0 bash/bash::sv_xtracefd | 0 | 11 |
| off perfect | O0 e2fsprogs/e2fsck::e2fsck_simple_progress | 0 | 11 |
| off perfect | O0 openssh-portable/ssh-keygen::write_checkpoint | 0 | 11 |
| off perfect | O0 shadow/faillog::print_one | 0 | 11 |
| off perfect | O0 openssh-portable/scp::sshbuf_free | 0 | 10 |
| off perfect | O0 openssh-portable/sftp::sshbuf_free | 0 | 10 |
| off perfect | O0 openssh-portable/sftp-server::sshbuf_free | 0 | 10 |
