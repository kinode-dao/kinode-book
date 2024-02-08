# In-Depth Guide: Chess App

This guide will walk you through building a very simple chess app on Kinode OS.
The final result will look like [this](https://github.com/kinode-dao/kinode/tree/main/modules/chess): chess is in the basic runtime distribution so you can try it yourself.

To prepare for this tutorial, follow the environment setup guide [here](../my_first_app/chapter_1.md), i.e. [start a fake node](../my_first_app/chapter_1.md#booting-a-fake-kinode-node) and then, in another terminal, run:
```bash
kit new my_chess
cd my_chess
kit build
kit start-package -p 8080
```

Once you have the template app installed and can see it running on your testing node, continue...
