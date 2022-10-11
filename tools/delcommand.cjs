const { REST, Routes } = require("discord.js");
const env = require("../out/env.js");

const rest = new REST({ version: "10" }).setToken(env.TOKEN);

rest.put(
	Routes.applicationGuildCommands(
		process.env["T_DELCOMMAND_CLIENT"],
		process.env["T_DELCOMMAND_GUILD"]
	),
	{ body: [] }
)
	.then(() => console.log("Successfully deleted all guild commands."))
	.catch(console.error);
