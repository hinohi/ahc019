import argparse

from common import calc_score, Args


def main():
    p = argparse.ArgumentParser()
    p.add_argument('-n', type=int, required=True)
    p.add_argument('-d', type=int, required=True)
    p.add_argument('--mc-run', '-R', type=int, required=True)
    p.add_argument('--max-temperature', '-T', type=float, required=True)
    p.add_argument('--min-temperature', type=float, default=1e-4)
    p.add_argument('--erase-small-th', type=int, default=2)
    p.add_argument('--cut-off', type=float, default=3.0)
    args = Args(**vars(p.parse_args()))
    print(calc_score(args))


if __name__ == '__main__':
    main()
