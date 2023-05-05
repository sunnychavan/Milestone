'''
reader() processes the desired info ['all'] or a list of heuristic names for batches described the step number
'''
def reader(step, info):
    import re
    
    log_path = "milestone_exp2.log"
    
    read = False
    batch_num = step
    temp_result = ""
    result = []
    
    with open(log_path, "r") as log:
        for line in log:
            if read:
                temp_result += line
            
                if "}," in line:
                    read = False
                    
                    float_regex = re.compile(r'\d+\.\d+')
                    if info == ['all']:
                        lst = [float(match.group()) for match in re.finditer(float_regex, temp_result)]
                        fmt_str = "".join((str(x) + " ") for x in lst)
                        result.append((batch_num, fmt_str.strip()))
                    else:
                        pattern = ""
                        for i in info:
                            if pattern != "":
                                pattern += "|"
                            pattern += re.escape(str(i)+": ") + '\"\d+\.\d+\"'
                        temp_str = ""
                        for match in re.finditer(pattern, temp_result):
                            temp_str += match.group()
                        lst = [match.group() for match in re.finditer(float_regex, temp_str)]
                        fmt_str = "".join((str(x) + " ") for x in lst)
                        result.append((batch_num, fmt_str.strip()))
                        
                    batch_num += step
                    temp_result = ""
                
            if "Batch #" + str(batch_num) in line:
                read = True
                
    return result