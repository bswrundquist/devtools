---
name: kubernetes-debug
description: Use when a Kubernetes workload is misbehaving — CrashLoopBackOff, ImagePullBackOff, OOMKilled, Pending pods, failing probes, services not reachable, jobs not completing. Systematic triage from symptom to root cause.
tools: Bash, Read, Grep, Glob
---

# Kubernetes Debug

Work the ladder: pod status → describe → events → logs → exec. The answer is
almost always in `describe` or the **previous** container's logs.

## The Ladder

```bash
kubectl get pods -n <ns> -o wide                  # status, restarts, node
kubectl describe pod <pod> -n <ns>                # events at the bottom — read them
kubectl logs <pod> -n <ns> --previous             # the crashed container, not the new one
kubectl get events -n <ns> --sort-by=.lastTimestamp | tail -20
kubectl exec -it <pod> -n <ns> -- sh              # only after the above
```

## Status Table

| Status | Meaning | First command |
|--------|---------|---------------|
| `CrashLoopBackOff` | Container exits, k8s keeps restarting | `logs --previous` |
| `ImagePullBackOff` | Bad image ref, missing pull secret, no registry access | `describe` (exact error in events) |
| `OOMKilled` (exit 137) | Exceeded memory **limit** | `describe` → Last State; raise limit or fix leak |
| `Pending` | Unschedulable | `describe` → why: resources, taints, affinity, PVC |
| `Evicted` | Node pressure kicked it off | `describe`; check requests are set |
| `CreateContainerConfigError` | Missing ConfigMap/Secret it references | `describe` names the missing key |
| Running but not Ready | Readiness probe failing | `describe` probe config + app logs |
| `Completed`/Job stuck | Check `backoffLimit`, `activeDeadlineSeconds` | `kubectl get job -o yaml` |

## CrashLoopBackOff

1. `kubectl logs <pod> --previous` — the crash is in the *last* run.
2. Empty logs → container dies before the app logs anything: bad entrypoint,
   missing env var, wrong command. `describe` shows exit code:
   - **1/2**: app error — read the stack trace.
   - **137**: OOM or SIGKILL — check Last State says OOMKilled.
   - **126/127**: command not executable / not found.
3. Probe-induced: liveness probe killing a slow-starting app. Symptom:
   restarts with healthy-looking logs. Fix with `startupProbe` or a longer
   `initialDelaySeconds` — don't just extend the liveness timeout.

## Pending / Scheduling

`describe pod` tells you exactly why. Decode:

```bash
kubectl describe nodes | grep -A5 "Allocated resources"   # who's full
kubectl get nodes -o custom-columns='NAME:.metadata.name,TAINTS:.spec.taints[*].key'
```

- `Insufficient cpu/memory` — requests don't fit any node. Lower requests or
  add nodes; check for one pod requesting more than any node has.
- `node(s) had untolerated taint` — needs a toleration or different node pool.
- `unbound PersistentVolumeClaims` — PVC pending: wrong storage class, or a
  zonal volume in the wrong zone.

## Service Not Reachable

Work backwards from the Service:

```bash
kubectl get endpoints <svc> -n <ns>     # EMPTY endpoints = selector matches nothing,
                                        # or no pod is Ready
kubectl get pods -l <selector> -n <ns>  # does the selector actually match?
kubectl run tmp --rm -it --image=busybox -- sh   # then: wget -qO- http://<svc>:<port>
nslookup <svc>.<ns>.svc.cluster.local            # DNS from inside
```

Empty endpoints is 90% of "service is down": selector typo, wrong
`targetPort`, or readiness failing on every pod.

## Resources

- Always set **requests** (scheduling) and **limits** (protection).
  Memory limit = OOMKill threshold; CPU limit = throttling, not killing.
- `kubectl top pods -n <ns>` vs the limits: running at >90% of memory limit
  is an OOMKill waiting for a traffic spike.
- CPU throttling makes latency mysteries: check
  `container_cpu_cfs_throttled_periods_total` if metrics are available, or
  drop CPU limits for latency-sensitive services and rely on requests.

## Rules

- `describe` before `logs`, `--previous` before `logs`, events before exec.
- Never fix by deleting the pod — the Deployment recreates it and you've
  destroyed the evidence. Diagnose first; deleting is not a fix.
- Exit code 137 + OOMKilled: decide leak vs legitimate need before raising
  the limit, by watching `kubectl top` over time.
- A liveness probe that runs business logic causes restart storms — probes
  check "is the process alive", readiness checks "can it serve".
- Empty Service endpoints → check the selector before anything network-level.
- In managed clusters (GKE etc.), node-level mysteries (evictions, disk
  pressure) → check the node pool before blaming the workload.
- State what changed: image tag, config, node pool, traffic. Pods don't start
  failing spontaneously.
