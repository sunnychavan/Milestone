
LOGFILE = "../logs/exp3.log"
DATAPATH = "../data/heuristic_elo_correlation_data/exp3/Heuristics Elo Data"
import re
import os

# def read_log_file(log_file_path):
#     with open(log_file_path) as log_file:
#         for line in log_file:
#             if "Elos of batch" in line or "Weights {" in line:
#                 print(line.strip())

# def weights_line_breakdown(line, weights_dict, current_elo):
#     key_value_pairs = line.split('{ ')[1].split(' },')[0].split(', ')
#     for pair in key_value_pairs:
#         key, value = pair.split(': ')
#         # Remove double quotes from around the value
#         value = value.strip('"')
#         # Convert the value to float
#         value = float(value)
#         # Add the key-value pair to the dictionary
#         if key in weights_dict:
#             weights_dict[key].append((value, current_elo))
#         else:
#             weights_dict[key] = [(value, current_elo)]

#     return weights_dict


def add_heuristic_weights_no_elo(line, weights_dict):
    key_value_pairs = line.split('{ ')[1].split(' },')[0].split(', ')
    for heur_value in key_value_pairs:
        heur, value = heur_value.split(': ')
        # Remove double quotes from around the value
        value = value.strip('"')
        # Convert the value to float
        value = float(value)
        # Add the key-value pair to the dictionary
        if heur in weights_dict:
            weights_dict[heur].append((value, None))
        else:
            weights_dict[heur] = [(value, None)]

    return weights_dict


def add_elo_to_weights_dict(elo_list, weights_dict):
    weight_dict_keys = list(weights_dict.keys())
    first_key = weight_dict_keys[0]
    weights_index = len(weights_dict[first_key])-1
    for elo in reversed(elo_list):
        for heur in weight_dict_keys:
            value, elo_none = weights_dict[heur][weights_index]
            weights_dict[heur][weights_index] = (value, elo)
        weights_index -= 1
    
    return weights_dict


def check_weights_errors(weights_dict, num_agents):
    weight_dict_keys = list(weights_dict.keys())
    if len(weight_dict_keys) == 0:
        return weights_dict
    first_key = weight_dict_keys[0]
    if weights_dict[first_key][-1][1] is None:
        for heur in weight_dict_keys:
            weights_dict[heur] = weights_dict[heur][:-num_agents]
    
    return weights_dict


    

# def read_log_file(filename):
#     weights_dict = {}
#     num_agents = None 
#     pattern = r'\((\d+)\)'
#     with open(filename, "r") as f:
#         current_elo_list = None 
#         for i, line in enumerate(f):
#             # if i == 555:
#             #     print("THIS IS I: " + str(i) + " THIS IS LINE: " + line)
#             #     return weights_dict
#             if 'Using NUM_AGENTS environment variable' in line:
#                 match = re.search(pattern, line)
#                 if match:
#                     num_agents = int(match.group(1))
#             elif "Running batch" in line:
#                 for i in range(num_agents):
#                     weights_dict =  add_heuristic_weights_no_elo(next(f).strip(), weights_dict)                                 
#             elif "Elos of batch" in line:
#                 elos_str = line.split(": ")[1].strip()
#                 current_elo_list = [int(x) for x in elos_str[1:-1].split(",")]
#                 weights_dict = add_elo_to_weights_dict(current_elo_list, weights_dict)
#                 # current_elo_list = sorted(elos_list, reverse=True)[:5]  
            
        
#         return weights_dict
    

def read_log_file(filename):
    weights_dict = {}
    num_agents = None 
    pattern = r'\((\d+)\)'
    prev = False 
    with open(filename, "r") as f:
        current_elo_list = None 
        for i, line in enumerate(f):
            # if i == 553:
            #     print("THIS IS I: " + str(i) + " THIS IS LINE: " + line)
            #     return weights_dict
            if 'Using NUM_AGENTS environment variable' in line:
                match = re.search(pattern, line)
                if match:
                    num_agents = int(match.group(1)) 
            elif "Running batch" in line:
                prev = True 
                weights_dict = check_weights_errors(weights_dict, num_agents)
            elif "Weights {" in line and prev:
                weights_dict =  add_heuristic_weights_no_elo(line.strip(), weights_dict)                                         
            elif "Elos of batch" in line:
                elos_str = line.split(": ")[1].strip()
                current_elo_list = [int(x) for x in elos_str[1:-1].split(",")]
                weights_dict = add_elo_to_weights_dict(current_elo_list, weights_dict)
                prev = False 
                # current_elo_list = sorted(elos_list, reverse=True)[:5]  
            
        
        return weights_dict
        

def write_to_heuristic_data_files(weights_dict, data_path):
    for heuristic in weights_dict:    
        file_path = os.path.join(data_path, heuristic + ".txt")
        with open(file_path, 'w') as f:
              f.write("Heuristic_Value, Elo_Value \n")
              for (h_value, elo) in weights_dict[heuristic]:
                  f.write(f"{h_value}, {elo}\n")



weights_dict = read_log_file(LOGFILE)
# print(weights_dict)
write_to_heuristic_data_files(weights_dict, DATAPATH)

# write_to_heuristic_data_files({}, DATAPATH)
# # read_log_file(LOGFILE)