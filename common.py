from dataclasses import dataclass
import math
import uuid
import time
import json

import boto3

ARN = 'arn:aws:states:ap-northeast-1:169698630369:stateMachine:AHC019'


@dataclass
class Args:
    n: int
    d: int
    mc_run: int
    max_temperature: float
    min_temperature: float
    erase_small_th: int
    cut_off: float


def calc_score(args: Args) -> tuple[float, float, float]:
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
    time.sleep(7)
    while True:
        out = client.describe_execution(executionArn=execution_arn)
        match out['status']:
            case 'SUCCEEDED':
                return summary_score(json.loads(out['output']))
            case 'FAILED' | 'TIMED_OUT' | 'ABORTED':
                print(out)
                raise RuntimeError
            case 'RUNNING':
                time.sleep(1)
            case s:
                print(s)
                raise RuntimeError


def summary_score(outputs: list[dict]) -> tuple[float, float, float]:
    n = len(outputs)
    s = 0.0
    log_s = 0.0
    c = 0
    for o in outputs:
        score = o['score']
        s += score
        log_s += math.log(score)
        c += o['run_count']
    return s / n, log_s / n, c / n
