
LOGFILE = "./logs/exp2.log"
DATAPATH = "./data/heuristic_elo_correlation_data/exp2"
import re
import os

# def read_log_file(log_file_path):
#     with open(log_file_path) as log_file:
#         for line in log_file:
#             if "Elos of batch" in line or "Weights {" in line:
#                 print(line.strip())

def weights_line_breakdown(line, weights_dict, current_elo):
    key_value_pairs = line.split('{ ')[1].split(' },')[0].split(', ')
    for pair in key_value_pairs:
        key, value = pair.split(': ')
        # Remove double quotes from around the value
        value = value.strip('"')
        # Convert the value to float
        value = float(value)
        # Add the key-value pair to the dictionary
        if key in weights_dict:
            weights_dict[key].append((value, current_elo))
        else:
            weights_dict[key] = [(value, current_elo)]

    return weights_dict

    

def read_log_file(filename):
    with open(filename, "r") as f:
        current_elo_list = None 
        weights_dict = {}
        for line in f:
            if "Elos of batch" in line:
                elos_str = line.split(": ")[1].strip()
                elos_list = [int(x) for x in elos_str[1:-1].split(",")]
                current_elo_list = sorted(elos_list, reverse=True)[:5]
            if "completed with best agents" in line:
                for i in range(5):
                    weights_dict = weights_line_breakdown(next(f).strip(), weights_dict, current_elo_list[i])
        
        return weights_dict
        

def write_to_heuristic_data_files(weights_dict, data_path):
    for heuristic in weights_dict:    
        file_path = os.path.join(data_path, heuristic + ".txt")
        with open(file_path, 'w') as f:
              f.write("Heuristic_Value, Elo_Value \n")
              for (h_value, elo) in weights_dict[heuristic]:
                  f.write(f"{h_value}, {elo}\n")



weights_dict = read_log_file(LOGFILE)
write_to_heuristic_data_files(weights_dict, DATAPATH)

# write_to_heuristic_data_files({}, DATAPATH)
# # read_log_file(LOGFILE)