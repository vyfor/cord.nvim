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
        "writerOpts": {
          "headerPartial": "# {{#with context}}{{#if isServerUpdate}}⚙️ {{/if}}{{/with}}[{{currentTag}}]{{#if title}} {{title}}{{/if}}\n\n{{#if date}}_{{date}}_{{/if}}",
          "commitPartial": "* {{#if scope}}**{{scope}}**: {{/if}}{{subject}} ([#{{commit.short}}]({{@root.repository}}/commit/{{hash}}))\n",
          "transform": function (commit, context) {
            if (!context.context) context.context = {};

            if (commit.scope === 'server') {
              context.context.isServerUpdate = true;
            }

            return {};
          }
        },
        "presetConfig": {
          "types": [
            { type: 'feat', section: 'Features' },
            { type: 'feature', section: 'Features' },
            { type: 'fix', section: 'Bug Fixes' },
            { type: 'perf', section: 'Performance Improvements' },
            { type: 'revert', section: 'Reverts' },
            { type: 'docs', section: 'Documentation', hidden: true },
            { type: 'style', section: 'Styles', hidden: false },
            { type: 'chore', section: 'Miscellaneous Chores', hidden: true },
            { type: 'refactor', section: 'Code Refactoring', hidden: false },
            { type: 'test', section: 'Tests', hidden: true },
            { type: 'build', section: 'Build System', hidden: true },
            { type: 'ci', section: 'Continuous Integration', hidden: true }
          ]
        }
      }
    ],
    [
      "@semantic-release/github",
      {
        "assets": "dist/*"
      }
    ],
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
