"""The RE-friction loop: agents reverse-engineer crackmes with kuna and record where it fails them.

Runbook: docs/re-pipeline.md. Sibling of scripts/pipeline/ (the angr/decbench loop), which
this package reuses for worker scheduling, claims, PR opening and observability rather than
forking. Stdlib only, run as `PYTHONPATH=<repo> python3 -m scripts.repipe.<mod>`.
"""
