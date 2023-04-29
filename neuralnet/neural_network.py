def run_nn(state_repr):
    import joblib
    import numpy 
    import pandas 
    import sklearn
    from sklearn.neural_network import MLPRegressor


    regr = joblib.load('neuralnet/nn.joblib')

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

    state_input = numpy.reshape(state, (1,37))
    return float(regr.predict(pandas.DataFrame(state_input)))


if __name__ == "__main__":
    run_nn("b:b/bb/bbb/bbbb/3/4/3/4/3/wwww/www/ww/w")