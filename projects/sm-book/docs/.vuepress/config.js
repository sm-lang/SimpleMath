let path = require("path")

const locale_cn = {
    selectText: '选择语言',
    label: '简体中文',
    editLinkText: '在 GitHub 上编辑此页',
    serviceWorker: {
        updatePopup: {
            message: "发现新内容可用.",
            buttonText: "刷新"
        }
    },
    sidebar: {
        "/cn/": [
            {
                title: '入门',
                collapsable: false,
                children: [
                    ['basic/', '安装与运行'],
                    ['basic/literal', '基本语法'],
                    ['basic/operators', '运算优先级'],
                    ['basic/questions', '疑难解答'],
                    ['basic/shortcoming', '语法问题'],
                ]
            },
        ],
    }
}

const locale_en = {
    selectText: 'Languages',
    label: 'English',
    ariaLabel: 'Languages',
    editLinkText: 'Edit this page on GitHub',
    serviceWorker: {
        updatePopup: {
            message: "New content is available.",
            buttonText: "Refresh"
        }
    },
    sidebar: {
        "/en/": [
            {
                title: 'Tutorials',
                collapsable: false,
                children: [
                    ['basic/', 'Basic Concepts'],
                ]
            },
        ]
    }
}

module.exports = {
    dest: 'docs/.build',
    locales: {
        '/cn/': {
            lang: 'zh-CN',
            title: 'Simple Math',
            lastUpdated: 'Last Updated',
        },
        '/en/': {
            lang: 'en-US',
            title: 'Simple Math',
            lastUpdated: 'Last Updated',
        }
    },
    head: [
        ['link', { rel: 'shortcut icon', type: "image/x-icon", href: './favicon.png' }]
    ],
    themeConfig: {
        repo: 'Galaster/SimpleMath',
        editLinks: true,
        docsDir: 'projects/sm-book/docs',
        markdown: {
            lineNumbers: true
        },
        locales: {
            '/cn/': locale_cn,
            '/en/': locale_en,
        },
    },
    serviceWorker: true,
    markdown: {
        config: md => {
        }
    },
    plugins: [
        [
            'shiki',
            {
                theme: 'monokai',
                langs: [
                    {
                        id: 'sm',
                        scopeName: 'source.sm',
                        path: path.resolve(__dirname, 'public/sm.tmLanguage.json'),
                        aliases: []
                    }
                ]
            }
        ],
    ],
};
