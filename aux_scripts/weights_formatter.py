import re

'''
From the log file, input the string into the UNFORMATTED STRING VARIABLE. Make 
sure to keep the single quotes as the string identifier.
'''

UNFORMATTED_STRING = 'Weights { Piece Diff: "2.69", Middle Line Diff: "0.53", Impt Pieces: "0.39", Middle Prox: "0.45", Aggression Diff: "0.11", Passive Diff: "3.15", Centrality: "0.04", Anti Centrality: "0.20", Defended Hexes: "1.10", Defended Hexes Middle Prox: "0.72", Undefended Pieces: "0.10", Undefended Pieces Middle Prox: "0.45", Attack in-sync: "0.14", Limit Oppo Moves: "0.18", Straight Lines: "0.21", Straight Lines Middle Prox: "0.63", Aggr Pieces: "0.01", Aggr Pieces Middle Prox: "0.01", Aggr Pieces Anti Centrality: "0.04" },'

def format_weights_string(unformatted_string):
    float_regex = re.compile(r'\d+\.\d+')
    float_list = [float(match.group()) for match in re.finditer(float_regex, unformatted_string)]
    formatted_string = "".join((str(x) + " ") for x in float_list)
    print(formatted_string)


format_weights_string(UNFORMATTED_STRING)