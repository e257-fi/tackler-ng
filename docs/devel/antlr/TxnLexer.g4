/*
 * Tackler-NG 2016-2019
 * SPDX-License-Identifier: Apache-2.0
 */
lexer grammar TxnLexer;

UUID_NAME: 'uuid';
LOCATION_NAME: 'location';
GEO_NAME: 'geo';
TAGS_NAME: 'tags';

UUID_VALUE: HEX HEX HEX HEX  HEX HEX HEX HEX '-' HEX HEX HEX HEX '-' HEX HEX HEX HEX '-' HEX HEX HEX HEX '-' HEX HEX HEX HEX HEX HEX HEX HEX  HEX HEX HEX HEX;

DATE: DIGIT DIGIT DIGIT DIGIT '-' DIGIT DIGIT '-' DIGIT DIGIT;
TS: DATE 'T' TIME;
TS_TZ: TS TZ;

INT: DIGIT+;

NUMBER: '-'? (INT | FLOAT);

ID: NameStartChar (NameChar)*;

SUBID: (NameStartChar | DIGIT) (NameChar)*;

fragment TIME: DIGIT DIGIT ':' DIGIT DIGIT ':' DIGIT DIGIT ('.' DIGIT+)?;

fragment TZ: 'Z' | (('+' | '-') DIGIT DIGIT ':' DIGIT DIGIT);

fragment FLOAT: DIGIT+ '.' DIGIT+;

fragment
NameChar
   : NameStartChar
   | DIGIT
   | '_'
   | '-'
   | '\u00B7'
   | '\u0300'..'\u036F'
   | '\u203F'..'\u2040'
   ;

fragment
NameStartChar
   : '$' | '¢' | '£' | '¤' | '¥' // common currency symbols which are not in block 20A0-20CF
   | '\u00B5' //  Micro Sign
   | '\u00B9' | '\u00B2' | '\u00B3' // Superscript 1, 2, 3 (Latin-1 Supplement)
   | '\u00B0' // Degree Sign
   | '\u00BC' | '\u00BD' | '\u00BE' // Vulgar Fraction: 1/4, 1/2, 3/4 (Latin-1 Supplement)
   | 'A'..'Z' | 'a'..'z'
   | '\u00C0'..'\u00D6'
   | '\u00D8'..'\u00F6'
   | '\u00F8'..'\u02FF'
   | '\u0370'..'\u037D'
   | '\u037F'..'\u1FFF'
   | '\u200C'..'\u200D'
   | '\u2070'..'\u218F'
   | '\u2C00'..'\u2FEF'
   | '\u3001'..'\uD7FF'
   | '\uF900'..'\uFDCF'
   | '\uFDF0'..'\uFFFD'
   ;

fragment HEX: [a-fA-F0-9];

fragment DIGIT: [0-9];

QUOTE: '\'';
L_BRACE: '(';
R_BRACE: ')';
L_CURLY: '{';
R_CURLY: '}';
L_SQUARE: '[';
R_SQUARE: ']';
L_ANGLE: '<';
R_ANGLE: '>';
HASH: '#';
AT: '@';
EQUAL: '=';
SPACE: ' ';
TAB: '\t';
COMMA: ',';
SEMICOLON: ';';
COLON: ':';
NL: '\r'? '\n';

ANYCHAR : . ;
