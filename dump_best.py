import optuna

n = 100
for d in range(5, 15):
    study = optuna.load_study(
        study_name=f'{d}_{n}',
        storage='sqlite:///opt.db',
    )
    print(f'// {study.best_trial.value}')
    print(f'{d} => McParams {{')
    for key, value in study.best_trial.params.items():
        print(f'{key}: {value},')
    print('},')
