#!/usr/bin/env python3
# coding: utf8

import sys
import json
import functools

"""
# JavaScript Code:

let dom = document.getElementsByTagName('table')[0].tBodies[0];

let table = Array.prototype.map.call(dom.children, function(tr){
    return Array.prototype.reduce.call(tr.children, function (acc, td){
        let name = td.className;
        let value = td.textContent;
        
        if ( name === 'character' ) {
            if ( value.length > 1 && value[0] === ' ' ) {
                value = value.slice(1);
            }
            acc[name] = value;
        } else if ( name === 'named' ) {
            acc[name] = value.split(" ");
        } else if ( name === 'hex' ) {
            acc[name] = value;
        } else if ( name === 'dec' ) {
            acc[name] = value;
        } else if ( name === 'desc' ) {
            acc[name] = value.trim();
        } else {
            throw new Error('Ooops ...');
        }
        
        return acc;
    }, {})
});

table = table.map(function (item){
    let code_point = Number(item['dec'].replace("&#", "").replace(";", ""));
    item['code'] = code_point;
    
    return item;
});

table.sort(function(a, b){
    return a['code'] - b['code'];
});

let codes = table.map(function (item){
    return item['code'];
});

console.log(JSON.stringify(table));
console.log(JSON.stringify(codes));


"""


table = json.loads(open("table.json", "r").read())

def mk_codes():
    NUM = 10
    MAX_WIDTH = 8
    codes = list(map(lambda item: item['code'], table))

    output = "[\n    "
    idx = 0
    while idx < len(codes):
        if idx > 0 and idx % NUM == 0:
            output += "\n    "

        s = "%d, " % codes[idx]
        output += s.rjust(MAX_WIDTH)

        idx += 1

    output = output[:-2]

    output += "\n]"

    return output


def mk_ranges():
    codes = list(map(lambda item: item['code'], table))

    ranges = [ ]

    start = codes[0]
    last = codes[0]

    for current in codes:
        if last == current:
            pass
        elif last + 1 != current:
            ranges.append((start, last))
            last = current
            start = current
        else:
            last = current

    last_range = ranges[-1]
    if last_range[0] != start:
        ranges.append((start, last))

        return ranges

def mk_named():
    output = []

    for item in table:
        named = item['named']
        char = chr(item['code'])

        for name in named:
            output.append( (name, char) )

    output = sorted(output, key=lambda item: item[0])

    MINIMAL_ENTITIES = ['"', "'", '&', '\\', '<', '>']
    minimal = list(filter(lambda item: item[1].isspace() or item[1] in MINIMAL_ENTITIES, output))
    
    def to_rust_char(c):
        if c == '\'':
            return "\\'"
        elif c == '\n':
            return "\\n"
        elif c == '\\t':
            return "\t"
        elif c == "\\":
            return r"\\"
        elif c == chr(8204):
            # &zwnj
            return "\\u{8204}"
        elif c.isspace():
            return "\\u{%d}" % ord(c)
        else:
            return c

    names = list(map(lambda item: item[0], output))
    max_name_length = max(list(map(lambda name: len(name), names)))
    for name in names:
        ident = name[1:-1]
        if not ident.isidentifier():
            print(name)
    # print(names)
    print(max_name_length)
    args = (
        max_name_length,
        len(minimal), ", ".join(map(lambda item: "(\"%s\", \'%s\')" % (item[0], to_rust_char(item[1])), minimal)),
        len(output), ", ".join(map(lambda item: "(\"%s\", \'%s\')" % (item[0], to_rust_char(item[1])), output)),
    )

    code = """

pub static MAX_NAME_LENGTH: usize = %d;

pub static MINIMAL_ENTITIES: [(&'static str, char); %d] = [
    %s
];

/// for decoder
pub static NAMED_ENTITIES: [(&'static str, char); %d] = [
    %s
];

""" % args
    
    return code


def main():
    d = list(map(lambda item: len(item['named']), table))
    print(max(d))


    # print("Length == 1:")
    # count_eq1 = 0
    # for item in table:
    #     if len(item['named']) == 1:
    #         # print(item)
    #         count_eq1 += 1
    # print(count_eq1)

    # print("Length > 2:")
    # count_gt2 = 0
    # for item in table:
    #     if len(item['named']) > 2:
    #         # print(item)
    #         count_gt2 += 1
    # print(count_gt2)

    # count_2 = 0
    # print("Length 2:")
    # for item in table:
    #     if len(item['named']) == 2:
    #         # print(item)
    #         count_2 += 1
    # print(count_2)

    # print(mk_codes())
    decode_table = mk_named()
    # print(decode_table)
    

if __name__ == '__main__':
    main()