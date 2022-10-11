require("dotenv").config();

import { Client, GatewayIntentBits } from "discord.js"

const { Guilds, MessageContent, GuildMessages, GuildMembers } = GatewayIntentBits
const client = new Client({ intents: [Guilds, MessageContent, GuildMessages, GuildMembers] })

client.login(process.env.TOKEN)