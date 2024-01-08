# Putting Everything Together

After adding a frontend in the previous chapter, your chess game is ready to play.

Hopefully, you've been using `uqdev build <your_chess_app_name>` to test the code as the tutorial has progressed.
If not, do so now in order to get a compiled package we can install onto a node.

Next, use `uqdev start-package <your_chess_app_name> --url <your_test_node_url>` to install the package.
You should see the printout we added to `init()` in your terminal: `chess by <your_node_name>: start`.

Remember that you determine the process name, package name, and your developer name in the `manifest.json` and `metadata.json` files inside `/pkg`.
Open your chess frontend by navigating to your node's URL (probably something like `http://localhost:8080`), and use the names you chose as the path.
For example, if your chess process name is `my_chess`, and your package is named `my_chess`, and your publisher name is `template.uq`, you would navigate to `http://localhost:8080/my_chess:my_chess:template.uq`.

You should see something like this:
![chess frontend](./chess_home.png)

To try it out, boot up another node, execute the `uqdev start-package` command, and invite your new node to a game.
Presto!

This concludes the main Chess tutorial.
If you're interested in learning more about how to write Uqbar processes, there are several great options to extend the app:

- Consider how to handle network errors and surface those to the user
- Add game tracking to the processes state, such that players can see their history
- Consider what another app might look like that uses the chess engine as a library.
Alter the process to serve this use case, or add another process that can be spawned to do such a thing.

There are also three extensions to this tutorial which dive into specific use cases which make the most of Uqbar:

- [Chat](./chat.md)
- [Payment Integration (using ETH)](./payment.md)
- [LLM Integration (play chess against the AI!)](./llm.md)

The full code is available [here](https://github.com/uqbar-dao/uqbar/tree/main/modules/chess).
