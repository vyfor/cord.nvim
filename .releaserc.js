module.exports = {
  "branches": [
    "master",
    {
      "name": "client-server",
      "prerelease": "beta",
      "channel": "beta"
    }
  ],
  "repositoryUrl": "https://github.com/vyfor/cord.nvim",
  "plugins": [
    "@semantic-release/commit-analyzer",
    [
      "@semantic-release/release-notes-generator",
      {
        "preset": "angular",
        "writerOpts": {
          "headerPartial": "# {{#with context}}{{#if isServerUpdate}}⚙️ {{/if}}{{/with}}[{{currentTag}}]{{#if title}} {{title}}{{/if}}\n\n{{#if date}}_{{date}}_{{/if}}",
          "transform": function (commit, context) {
            if (!context.context) context.context = {};

            if (commit.scope === 'server') {
              context.context.isServerUpdate = true;
            }

            console.log(commit);
            const shortHash = commit.hash.substring(0, 7);

            return {
              type: commit.type,
              scope: commit.scope,
              subject: commit.subject || '',
              hash: shortHash,
              shortDesc: commit.subject || '',
              body: commit.body || '',
              footer: commit.footer || '',
              notes: commit.notes || []
            };
          }
        }
      }
    ],
    "@semantic-release/changelog",
    "@semantic-release/github",
    [
      "@semantic-release/git",
      {
        "assets": [
          "CHANGELOG.md"
        ],
        "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
      }
    ]
  ]
}