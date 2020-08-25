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
                title: '概览',
                collapsable: false,
                children: [
                    ['starter/', '安装与运行'],
                    ['starter/literal', '基本语法'],
                    ['starter/function', '函数调用'],
                    ['starter/repl', '交互式环境'],
                ]
            },
            {
                title: '入门',
                collapsable: false,
                children: [
                    ['basic/', '其他语法'],
                    ['basic/function', '函数综述'],
                    ['basic/questions', '疑难解答'],
                ]
            },
            {
                title: '进阶',
                collapsable: false,
                children: [
                    ['advance/', '进阶知识'],
                ]
            },
            {
                title: '开发',
                collapsable: false,
                children: [
                    ['deveoper/operators', '运算优先级'],
                    ['deveoper/shortcoming', '语法问题'],
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
                    },
                    {
                        id: 'wolfram',
                        scopeName: 'source.wolfram',
                        path: path.resolve(__dirname, 'public/wl.tmLanguage.json'),
                        aliases: [
                            'wl'
                        ]
                    }
                ]
            }
        ],
    ],
};
