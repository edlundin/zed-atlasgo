(comment) @comment

(block (identifier) @type)
(attribute (identifier) @property)

(identifier) @variable
(function_call (identifier) @function)

(string_lit) @string
(template_literal) @string
(heredoc_template) @string
(numeric_lit) @number
(bool_lit) @boolean
(null_lit) @constant.builtin

[
  "for"
  "in"
  "if"
] @keyword

[
  "="
  "."
  ","
  "=>"
] @operator

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket
