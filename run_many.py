import json
import argparse
import uuid
import time
from pathlib import Path
from datetime import datetime
import math

import boto3

ARN = 'arn:aws:states:ap-northeast-1:169698630369:stateMachine:AHC019'


def main():
    p = argparse.ArgumentParser()
    p.add_argument('-n', type=int, required=True)
    p.add_argument('-d', type=int, required=True)
    p.add_argument('--mc-run', '-R', type=int, required=True)
    p.add_argument('--max-temperature', '-T', type=float, required=True)
    p.add_argument('--min-temperature', type=float, default=1e-4)
    p.add_argument('--erase-small-th', type=int, default=2)
    p.add_argument('--cut-off', type=float, default=3.0)
    args = p.parse_args()

    tasks = []
    for seed in range(args.n):
        tasks.append({
            'seed': seed + 1,
            'd': args.d,
            'mc_run': args.mc_run,
            'max_temperature': args.max_temperature,
            'min_temperature': args.min_temperature,
            'erase_small_th': args.erase_small_th,
            'cut_off': args.cut_off,
        })

    client = boto3.client('stepfunctions')
    name = str(uuid.uuid4())
    r = client.start_execution(
        stateMachineArn=ARN,
        name=name,
        input=json.dumps(tasks),
    )
    execution_arn = r['executionArn']
    print(execution_arn)
    time.sleep(7)
    while True:
        out = client.describe_execution(executionArn=execution_arn)
        match out['status']:
            case 'SUCCEEDED':
                summary_score(name, json.loads(out['output']))
                break
            case 'FAILED' | 'TIMED_OUT' | 'ABORTED':
                print(out)
                break
            case 'RUNNING':
                time.sleep(1)
            case s:
                print(s)


def summary_score(name: str, outputs: list[dict]):
    base = Path(__file__).parent / 'exec_log'
    base.mkdir(parents=True, exist_ok=True)
    fname = f'{datetime.now():%m%d_%H%M%S}_{name}.json'
    with (base / fname).open('w') as f:
        json.dump(outputs, f, indent=2)

    params = outputs[0]['request']
    d = params['d']
    n = len(outputs)

    s = 0.0
    log_s = 0.0
    c = 0
    for o in outputs:
        score = o['score']
        s += score
        log_s += math.log(score)
        c += o['run_count']
    s /= n
    log_s /= n
    c /= n
    print(f'{d} {n} {c} {s} {log_s}')


if __name__ == '__main__':
    main()
