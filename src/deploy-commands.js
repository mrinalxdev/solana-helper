const { Routes, REST } = require("discord.js");
require("dotenv").config();

const token = process.env.BOT_TOKEN;
const rest = new REST().setToken();

const commands = [];

(async () => {
  try {
    console.log(
      `Started refreshing ${commands.length} application (/) commands.`
    );

    const data = await rest.put(
      Routes.applicationGuildCommands(clientId, guildId),
      { body: commands }
    );

    console.log(
      `Successfully reloaded ${data.length} application (/) commands`
    );
  } catch (error) {
    console.log(error);
  }
})();
