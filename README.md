# REgex Query #
This is the Rust-base command-line regex processor.

![GitHub Action](https://github.com/cmj0121/req/workflows/pipeline/badge.svg)

---

`req` supports parse the text via [regular expression][regex] and generate the JSON-like object,
which include the matched string, sub-groups and named groups.

## Query Mode ##
In more detail the req support two type of match method and three-kinds of query mnode. The match
modes can be *single* and *all* which single will return the first match result. And there are
three query node: *full*, *group* and *named*. The full-mode will return all the matched string,
no matter the regular pattern contain the sub-group or named group. In the group-mode it will only
return the sub-group described in the pattern. The named-mode, like the group-mode, only reply
the named group.



[regex]: https://en.wikipedia.org/wiki/Regular_expression

