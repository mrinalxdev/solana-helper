const {Client, GatewayIntentBits} = require("discord.js");
require("dotenv").config();

const TOKEN = process.env.BOT_TOKEN;
const clientId = process.env.CLIENT_ID;
const guildId = process.env.GUILD_ID;

const client = new Client({
  intents : [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMembers,
    GatewayIntentBits.GuildVoiceStats,
  ],
  
});

(async () => {
  await client.login(TOKEN);
  console.log(`${client.user.tag} is online!`)
})();
