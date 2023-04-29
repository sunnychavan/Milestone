import joblib
import numpy as np
import pandas as pd

def run_nn(state_repr):
    regr = joblib.load('nn.joblib')

    state = []
    for i in state_repr[2:]:
        if i == 'b':
            state.append(2)
        elif i == 'w':
            state.append(1)
        elif i == '/':
            continue
        else:
            for j in range(int(i)):
                state.append(0)

    state_input = np.reshape(state, (1,37))
    return float(regr.predict(pd.DataFrame(state_input)))
    

