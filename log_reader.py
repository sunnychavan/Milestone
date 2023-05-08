import re


'''
reader() processes the desired info ['all'] or a list of heuristic names for batches described the step number
'''
def reader(log_path, step, info):

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

if __name__ == "__main__":
    with open("data/exp2.agents", "w") as f:
        agents = reader("logs/exp2.log", 4, ["all"])
        for agent in agents:
            f.write(str(agent[0]) + ',' + agent[1] + '\n')

    with open("data/exp3.agents", "w") as f:
        agents = reader("logs/exp3.log", 4, ["all"])
        for agent in agents:
            f.write(str(agent[0]) + ',' + agent[1] + '\n')
