"use strict";
exports.__esModule = true;
var fs = require("fs");
var removeTS = function (name) { return name.replace('.ts', ''); };
fs.readdir("./bindings/enums", function (_, files) {
    var header = "// THIS CODE WAS AUTO-GENERATED. DO NOT EDIT MANUALLY.\n\n";
    var enums = files
        .filter(function (f) { return f.endsWith(".ts"); })
        .map(function (f) { return removeTS(f); })
        .filter(function (f) { return f !== 'types'; });
    var imports = enums.map(function (f) { return "import {".concat(f, "} from './enums/").concat(f, "';"); });
    var messageTypes = "";
    enums.forEach(function (e) {
        var types = fs.readdirSync("./bindings/messageTypes/" + e)
            .filter(function (f) { return f.endsWith(".ts"); })
            .map(function (f) { return removeTS(f); })
            .filter(function (f) { return f !== 'types'; });
        imports.push.apply(imports, types.map(function (f) { return "import {".concat(f, "} from './messageTypes/").concat(e, "/").concat(f, "';"); }));
        messageTypes += "export interface ".concat(e, "Types { \n");
        messageTypes += types.map(function (t) { return "\t'".concat(t, "' : ").concat(t, ";"); }).join("\n");
        messageTypes += '\n}\n';
        messageTypes += "export type ClientMessageType = Pick<ClientMessageTypes, keyof ClientMessageTypes>;\n";
    });
    var handlerInterfaces = enums.map(function (f) { return "export type ".concat(f, "Handler = {\n\t[key in keyof ").concat(f, "Types]: (arg: ").concat(f, "Types[key]) => void;\n}"); }).join("\n");
    var final = "".concat(header).concat(imports.join('\n'), "\n\n").concat(messageTypes, "\n\n").concat(handlerInterfaces);
    fs.writeFileSync('./bindings/types.ts', final);
});
