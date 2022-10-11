import * as _ from "dotenv";

import { Client, GatewayIntentBits, REST, Routes } from "discord.js";
import { emoji, execute as executeEmoji } from "./emoji";
import * as env from "./env";

// Reload and update slash commands
(async () => {
	const rest = new REST({ version: "10" }).setToken(env.TOKEN);
	try {
		console.log("Started refreshing application (/) commands.");

		await rest.put(Routes.applicationCommands("1029453218465456270"), {
			body: [emoji],
		});

		console.log("Successfully reloaded application (/) commands.");
	} catch (error) {
		console.error(error);
	}
})();

const { Guilds, MessageContent, GuildMessages, GuildMembers } =
	GatewayIntentBits;

const client = new Client({
	intents: [Guilds, MessageContent, GuildMessages, GuildMembers],
});

client.on("ready", () => {
	// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
	console.log(`Logged in as ${client.user!.tag}`);
});

client.on("interactionCreate", async interaction => {
	if (!interaction.isChatInputCommand()) return;

	if (interaction.commandName === "emoji") await executeEmoji(interaction);
});

client.login(env.TOKEN);
