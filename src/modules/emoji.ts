import { CommandInteraction } from "discord.js";
import { Module } from ".";
import Logger from "../log";
import { registerSlashCommand } from "../util/cmd";

export default class implements Module {
	name = "emoji";
	subscribedTo = ["ready", "interactionCreate"];
	logger = new Logger(this.name);

	async ready() {
		await registerSlashCommand({
			name: "emoji",
			description: "Emoji menüsünü açar",
		});
	}

	async interactionCreate(inter: CommandInteraction) {
		if (!inter.isCommand()) return;
		if (inter.commandName != "emoji") return;

		await inter.reply("todo!");
	}
}
