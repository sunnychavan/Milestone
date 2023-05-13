# %%
import numpy as np
import pandas as pd


# %%
games = pd.read_csv("game_table.txt")
states = pd.read_csv("state_table.txt", usecols=['state_id', 'state', 'game_id'])

# %%
games

# %%
states

# %%
joined = states.merge(games, on="game_id")

# %%
joined

# %%
grouped = joined.groupby(by=["state"], as_index=False).agg({"result": ['sum', 'count']})

# %%
grouped

# %%
grouped['black_win_ratio'] = 1-grouped['result']['sum']/grouped['result']['count']

# %%
grouped

# %%
def transform_state(state):
    board = []
    for i in state[2:]:
        if i == 'b':
            board.append(2)
        elif i == 'w':
            board.append(1)
        elif i == '/':
            continue
        else:
            for j in range(int(i)):
                board.append(0)

    return board

def split_array(arr):
    return pd.Series(arr)

# %%
transformed_states = grouped['state'].apply(transform_state)

# %%
new_cols = pd.DataFrame(transformed_states.tolist())

# %%
new_cols

# %%
import sklearn
from sklearn.neural_network import MLPRegressor

# %%
X_train, X_test, y_train, y_test = sklearn.model_selection.train_test_split(new_cols, grouped['black_win_ratio'])

# %%
X_train

# %%
X_test

# %%
regr = MLPRegressor().fit(X_train, y_train)

# %%
from sklearn.metrics import mean_squared_error

mean_squared_error(y_test, regr.predict(X_test))

# %%
import joblib

# %%
joblib.dump(regr, 'exp3.joblib')

# %%
test_state = transform_state("b:b/bb/bbb/bbbb/3/4/3/4/3/wwww/www/ww/w")
test_state = np.reshape(test_state, (1,37))
test_input = pd.DataFrame(test_state)
test_input

# %%
