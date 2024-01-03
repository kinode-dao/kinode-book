# Chapter 5: Sharing With The World

So, you've made a new process.
You've tested your code and are ready to share with friends, or perhaps just install across a couple nodes in order to do more testing.

First, it's a good idea to publish the code to a public repository.
This can be added to your package metadata.json like so:
```json
...
"website": "https://github.com/your_package_repo",
...
```

Next, review all the data in `pkg/manifest.json` and `pkg/metadata.json`.
The `package` field in metadata determines the name of the package.
The `publisher` field determines the name of the publisher (you!).
**Note: you *can* set any publisher name you want, but others will be able to verify that you are the publisher by comparing the value in this field with a signature attached to the entry in a (good) app store or package manager, so it's a good idea to put *your node identity* here.**

(TODO: talk about how to sign packages)

Once you're ready to share, it's quite easy.
If you are developing on a fake node, you'll have to boot a real one, then install this package locally in order to publish on the network.
If you're already on a real node, you can go ahead and navigate to the App Store on the homepage and go through the publishing flow.

(TODO: complete this page when the app_store frontend is ready)
