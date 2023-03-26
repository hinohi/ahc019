import argparse
import optuna

from common import Args, calc_score


def main():
    p = argparse.ArgumentParser()
    p.add_argument('-d', type=int, required=True)
    p.add_argument('-n', type=int, required=True)
    p.add_argument('--trials', type=int, default=100)
    args = p.parse_args()
    d: int = args.d
    n: int = args.n

    study = optuna.create_study(
        study_name=f'{d}_{n}',
        storage='sqlite:///opt.db',
        load_if_exists=True,
        direction=optuna.study.StudyDirection.MINIMIZE,
    )

    def objective(trial: optuna.Trial):
        score_args = Args(
            n=n,
            d=d,
            mc_run=trial.suggest_int('mc_run', low=1, high=100),
            max_temperature=trial.suggest_float('max_temperature', low=1e-3, high=200.0, log=True),
            min_temperature=1e-8,
            erase_small_th=trial.suggest_int('erase_small_th', low=0, high=60),
            erase_shared_p=trial.suggest_float('erase_shared_p', low=0.0, high=1.0),
            cut_off=trial.suggest_float('cut_off', low=0.5, high=10.0),
        )
        score = calc_score(score_args)
        print(score)
        return score[1]

    study.optimize(objective, n_trials=args.trials)


if __name__ == '__main__':
    main()
