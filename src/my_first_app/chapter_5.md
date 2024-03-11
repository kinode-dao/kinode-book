# Sharing with the World

So, you've made a new process.
You've tested your code and are ready to share with friends, or perhaps just install across multiple nodes in order to do more testing.

First, it's a good idea to publish the code to a public repository.
This can be added to your package metadata.json like so:
```json
...
"website": "https://github.com/your_package_repo",
...
```

Next, review all the data in [`pkg/manifest.json`](./chapter_1.md#pkgmanifestjson) and [`metadata.json`](./chapter_1.md#pkgmetadatajson).
The `package_name` field in `metadata.json` determines the name of the package.
The `publisher` field determines the name of the publisher (you!).

**Note: you *can* set any publisher name you want, but others will be able to verify that you are the publisher by comparing the value in this field with a signature attached to the entry in a (good) app store or package manager, so it's a good idea to put *your node identity* here.**

Once you're ready to share, it's quite easy.
If you are developing on a fake node, you'll have to boot a real one, then install this package locally in order to publish on the network.
If you're already on a real node, you can go ahead and navigate to the App Store on the homepage and go through the publishing flow.

In the near future, you will be able to quickly and easily publish your applications to the network using a GUI from the App Store.

Right now, you can deploy your app to the network by following the steps in the next section.

## Ad-Hoc App Deployment

While the App Store GUI is under development, this series of steps will allow you to deploy your app to the network.
Soon, it will be a lot easier!

1. Install the app on your node.
1. In your terminal, navigate to `<your_node_dir>/vfs/<your_package>/pkg`.
1. Hash the .zip file with SHA256: `sha256sum <your_package>.zip`
1. Add the hash to your package's [`metadata.json`](./chapter_1.md#pkgmetadatajson), under `properties` -> `code_hashes` -> `<app_version>`.
1. Save the `metadata.json` file and ensure it is hosted somewhere on the internet accessible via URL.
For GitHub repositories, you can access the file's raw contents at the following link: `https://raw.githubusercontent.com/<your_package_repo>/main/pkg/metadata.json`
1. Navigate to the App Store on the homepage and click "Publish" (the upload icon).
1. Enter your package information, passing the URL of your `metadata.json` file in the `Metadata URL` field.
1. Click "Publish".

Congratulations, your app is now live on the network!
