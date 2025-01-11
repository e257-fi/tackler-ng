#!/usr/bin/python3
# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# Tackler-NG 2016-2024 Contributors
#
# SPDX-License-Identifier: Apache-2.0
#############################################################################

#versions = [ "24.12.1", "24.12.2" ]
versions = ["24.12.1", "24.12.2", "devel"]

#
# Plot perf data with Gnuplot
# Usage:
#   sh bin/perf-all.sh
#   sh bin/perf-data-migration.sh results/hwXX 24.12.1 1E3 > results/hw02/hw02-24.12.1-1E3.yml
#   sh bin/perf-data-migration.sh results/hwXX 24.12.1 1E6 > results/hw02/hw02-24.12.1-1E6.yml
#   python3 bin/perf-plot.py results hw02 1E3 | gnuplot
#   python3 bin/perf-plot.py results hw02 1E6 | gnuplot
#

import yaml
import argparse
import sys


def make_xtics(versions):
    result_str = "set xtics ("
    result_str += '"' + versions[0] + '" 1'
    i = 2
    for v in versions[1:]:
        if v == "devel":
            v = "XX.YY.Z"

        result_str += ', "' + v + '" ' + "{:d}".format(i)
        i = i + 1

    result_str += ")"
    return (result_str, i)

def plot_def(hw, testset):
    return """
    #
    #
    set term svg dashed size 2400,1600 dynamic background "0xFFFFFF"
    set output "perf-%s-%s.svg"
    set size 1.0,1.0
    set origin 0.0,0.0
    set xtics rotate
    set multiplot
    #
    #
    """ % (hw, testset)


def plot_line_def():
    #'-' using 2:xtic(1) t "balance (json, all)"        with linespoints pt 7 lc rgbcolor "0x00FF00" lw 2 dt 2, \
    #'-' using 2:xtic(1) t "balance-group (json, all)"  with linespoints pt 7 lc rgbcolor "0x0000FF" lw 2 dt 2, \
    #'-' using 2:xtic(1) t "register (json, all)"       with linespoints pt 7 lc rgbcolor "0xFF0000" lw 2 dt 2, \
    #'-' using 2:xtic(1) t "balance (json, flt)"        with linespoints pt 7 lc rgbcolor "0x008800" lw 2 dt 3, \
    #'-' using 2:xtic(1) t "balance-group (json, flt)"  with linespoints pt 7 lc rgbcolor "0x000088" lw 2 dt 3, \
    #'-' using 2:xtic(1) t "register (json, flt)"       with linespoints pt 7 lc rgbcolor "0x880000" lw 2 dt 3

    return """
    plot \
    '-' using 2:xtic(1) t "balance (txt, all)"         with linespoints pt 9 lc rgbcolor "0x00FF00" lw 2 dt 1, \
    '-' using 2:xtic(1) t "balance-group (txt, all)"   with linespoints pt 9 lc rgbcolor "0x0000FF" lw 2 dt 1, \
    '-' using 2:xtic(1) t "register (txt, all)"        with linespoints pt 9 lc rgbcolor "0xFF0000" lw 2 dt 1, \
    '-' using 2:xtic(1) t "bal+reg  (txt, flt)"        with linespoints pt 9 lc rgbcolor "0xFF00FF" lw 2 dt 1, \
    '-' using 2:xtic(1) t "balance (txt, flt)"         with linespoints pt 9 lc rgbcolor "0x00CC00" lw 2 dt 4, \
    '-' using 2:xtic(1) t "balance-group (txt, flt)"   with linespoints pt 9 lc rgbcolor "0x0000CC" lw 2 dt 4, \
    '-' using 2:xtic(1) t "register (txt, flt)"        with linespoints pt 9 lc rgbcolor "0xCC0000" lw 2 dt 4, \
    '-' using 2:xtic(1) t "bal+reg  (txt, flt)"        with linespoints pt 9 lc rgbcolor "0xCC00CC" lw 2 dt 4
    """

def storage_plot_line_def():
    return """
    plot \
    '-' using 2:xtic(1) t "FS (balance, txt)"    with linespoints pt 9 lc rgbcolor "0xFF0000" lw 2 dt 1, \
    '-' using 2:xtic(1) t "GIT  (balance, txt)"  with linespoints pt 7 lc rgbcolor "0x0000FF" lw 2 dt 1
    """


def plot_time(testset):
    xtics = make_xtics(versions)

    p_hdr = """
    set size 0.33,0.5
    set origin 0,0.5
    set grid
    set title "Reporting - Set: %s"
    set key top left
    set ylabel "Time (s)"
    set xrange  [0:%d]
    set yrange [*:*]
    """ % (testset, len(xtics))

    return p_hdr + xtics[0] + plot_line_def()


def plot_mem(testset):
    xtics = make_xtics(versions)

    p_hdr = """
    set size 0.33,0.5
    set origin 0.33,0.5
    set grid
    set title "Reporting - Set: %s"
    set key top left
    set ylabel "Memory (M)"
    set xrange  [0:%d]
    set yrange [*:*]
    """ % (testset, len(xtics))

    return p_hdr + xtics[0] + plot_line_def()


def plot_cpu(testset):
    xtics = make_xtics(versions)

    p_hdr = """
    set size 0.33,0.5
    set origin 0.66,0.5
    set grid
    set title "Reporting - Set: %s"
    set key top left
    set ylabel "CPU %%"
    set xrange  [0:%d]
    set yrange [*:*]
    """ % (testset, len(xtics))

    return p_hdr + xtics[0] + plot_line_def()

###
### GIT vs. FS
###
def storage_plot_time(testset):
    xtics = make_xtics(versions)

    p_hdr = """
    set size 0.33,0.5
    set origin 0,0
    set grid
    set title "Storage - Set: %s"
    set key top left
    set ylabel "Time (s)"
    set xrange  [0:%d]
    set yrange [*:*]
    """ % (testset, len(xtics))

    return p_hdr + xtics[0] + storage_plot_line_def()


def storage_plot_mem(testset):
    xtics = make_xtics(versions)

    p_hdr = """
    set size 0.33,0.5
    set origin 0.33,0
    set grid
    set title "Storage - Set: %s"
    set key top left
    set ylabel "Memory (M)"
    set xrange  [0:%d]
    set yrange [*:*]
    """ % (testset, len(xtics))

    return p_hdr + xtics[0] + storage_plot_line_def()


def storage_plot_cpu(testset):
    xtics = make_xtics(versions)

    p_hdr = """
    set size 0.33,0.5
    set origin 0.66,0
    set grid
    set title "Storage - Set: %s"
    set key top left
    set ylabel "CPU %%"
    set xrange  [0:%d]
    set yrange [*:*]
    """ % (testset, len(xtics))

    return p_hdr + xtics[0] + storage_plot_line_def()


def values_average(values):
    l = values.get("values")
    assert (len(l) == 5)
    l.sort()

    return sum(l[1:4]) / 3


def gnuplot_version(version, dev=False):
    if dev or version == "devel":
        return "XX.YY.Z"
    else:
        return version


def values_to_plot(data, key, value_getter, v_func):
    # find result set (times, mem, cpu), based on key triplet (report, format, filter)
    def find_result():
        for run in runs:
            r = run.get("run")
            if r.get("report") == rpt and \
                    r.get("formats") == frmt and \
                    (len(r.get("filter")) != 0) == flt:
                return r

    # get wanted value with value_getter, convert value with v_func and add it to the plot
    def value_to_plot(last):
        try:
            result = find_result()
            value = values_average(value_getter(result.get("result"), key))
            return gnuplot_version(v, last) + "  " + "{:.2f}".format(v_func(value)) + "\n"
        except AttributeError:
            return ""

    result_str = ""
    for flt in [False, True]:
        #for frmt in ["txt", "json"]:
        for frmt in ["txt"]:
            for rpt in ["balance", "balance-group", "register", "balance_register"]:
                for v in versions:
                    version_data = data.get(v)
                    if version_data:
                        runs = data.get(v).get("runs")
                        result_str += value_to_plot(False)

                result_str += "e\n"

    return result_str


def storage_values_to_plot(data, key, value_getter, v_func):
    # find result set (times, mem, cpu), based on key triplet (report, format, filter)
    def find_result():
        for run in runs:
            r = run.get("run")
            if  r.get("storage") == storage and \
                r.get("report") == rpt and \
                    r.get("formats") == frmt and \
                    (len(r.get("filter")) != 0) == flt:
                return r

    # get wanted value with value_getter, convert value with v_func and add it to the plot
    def value_to_plot(last):
        try:
            result = find_result()
            value = values_average(value_getter(result.get("result"), key))
            return gnuplot_version(v, last) + "  " + "{:.2f}".format(v_func(value)) + "\n"
        except AttributeError:
            return ""

    result_str = ""
    for storage in ["fs", "git"]:
        for flt in [False]:
            for frmt in ["txt"]:
                for rpt in ["balance"]:
                    for v in versions:
                        version_data = data.get(v)
                        if version_data:
                            runs = data.get(v).get("runs")
                            result_str += value_to_plot(False)

                    result_str += "e\n"

    return result_str



def main():
    argp = argparse.ArgumentParser(description="tackler perf data plotter")

    argp.add_argument("basedir", help="path to basedir")
    argp.add_argument("hw", help="Test hardware")
    argp.add_argument("set", help="1E3 ...")
    args = argp.parse_args()

    data = dict()
    for v in versions:
        data_file = args.basedir + "/" + args.hw + "/" + args.hw + "-" + v + "-" + args.set + ".yml"
        try:
            with open(data_file, "r") as f:
                data[v] = yaml.safe_load(f.read())
        except FileNotFoundError:
            data[v] = None

    print(plot_def(args.hw, args.set))

    print(plot_time(args.set))
    print(values_to_plot(data, "real",
                         lambda z, key: z.get("times").get(key),
                         lambda x: x))

    print(plot_mem(args.set))
    print(values_to_plot(data, "mem",
                         lambda z, key: z.get(key),
                         lambda x: x / 1024))

    print(plot_cpu(args.set))
    print(values_to_plot(data, "cpu",
                         lambda z, key: z.get(key),
                         lambda x: x))

    print(storage_plot_time(args.set))
    print(storage_values_to_plot(data, "real",
                         lambda z, key: z.get("times").get(key),
                         lambda x: x))

    print(storage_plot_mem(args.set))
    print(storage_values_to_plot(data, "mem",
                         lambda z, key: z.get(key),
                         lambda x: x / 1024))

    print(storage_plot_cpu(args.set))
    print(storage_values_to_plot(data, "cpu",
                         lambda z, key: z.get(key),
                         lambda x: x))

    print("unset multiplot")


if __name__ == "__main__":
    main()
