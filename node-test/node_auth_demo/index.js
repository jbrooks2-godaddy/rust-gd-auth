import node_gd_auth_cobhan from 'node_gd_auth_cobhan';

let devToken = "eyJhbGciOiAiUlMyNTYiLCAia2lkIjogIjV5UXU0WlhXMUEifQ.eyJhdXRoIjogImJhc2ljIiwgImZ0YyI6IDEsICJpYXQiOiAxNjYyMTU5ODUzLCAianRpIjogIlhrSjNzOTE5Z3F1NVlZaW9oSW1QTnciLCAidHlwIjogImlkcCIsICJ2YXQiOiAxNjYyMTU5ODUzLCAiZmFjdG9ycyI6IHsia19wdyI6IDE2NjIxNTk4NTN9LCAicGVyIjogdHJ1ZSwgImhiaSI6IDE2NjIxNTk4NTMsICJzaG9wcGVySWQiOiAiMTY4MzE3MyIsICJjaWQiOiAiZTY2NWFlNGEtODE5OS00Yjg5LWI1ZmEtZTExOTFiYWZiZjY3IiwgInBsaWQiOiAiMSIsICJwbHQiOiAxLCAic2hhcmQiOiAiMDEwMiIsICJpZGVudGl0eSI6ICIwYjQ5NjlhMC1mMWI2LTExZTgtODM2OS0wMjQyYzBhOGIwMDIifQ.1c8Gw4I5j5-mqiY1gyMXHEPleAggUhB63-DrLqsRgiuzTe4YQ0Qk5VgDab1RphivU410rDMB2_jTLN4Sw6zYBPxnKcHa-a7rxzDmdzJ6kttxqckHyFhOWeqAwMfuYWtuVsu7mpBbyNroSW8hrMun-pYAoc1uvTvIUIIgfrgY7KBrPXaT6GtjH_Io2yW13ihYq3hO_I5TkzbnTVUBkteIS-t390EJQDb6gMMNQTzx5FI7uSC_klBIGMCQXBnRMXpzLQMEaTdIRafRD0utZ2dA4qrHMwVHmtAlKxRJgfLwsOboqRmWXCaWNAvJphncj_cjw26i_9XLMstYB6YsKNcuOQ"
let config = {
    "host": "dev-godaddy.com"
}
console.log("Parse token claims:")
console.log(node_gd_auth_cobhan.parse(config, devToken))