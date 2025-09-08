# 搜索文件

## OpenAPI Specification

```yaml
openapi: 3.0.1
info:
  title: ''
  description: ''
  version: 1.0.0
paths:
  /:
    get:
      summary: 搜索文件
      deprecated: false
      description: ''
      tags: []
      parameters:
        - name: search
          in: query
          description: 搜索关键字
          required: true
          example: index.css
          schema:
            type: string
        - name: json
          in: query
          description: 使用json
          required: false
          example: '1'
          schema:
            type: string
        - name: offset
          in: query
          description: 页码-1
          required: false
          example: '0'
          schema:
            type: string
        - name: count
          in: query
          description: 页大小
          required: false
          example: '20'
          schema:
            type: string
        - name: sort
          in: query
          description: "排序，name 名称，path\t路径，date_modified 修改日期，size 大小"
          required: false
          example: date_modified
          schema:
            type: string
        - name: case
          in: query
          description: 0：不匹配大小写，非0：匹配
          required: false
          example: '0'
          schema:
            type: string
        - name: wholeword
          in: query
          description: 0：不匹配全字，非0：匹配
          required: false
          example: '0'
          schema:
            type: string
        - name: path
          in: query
          description: 0：不匹配路径，非0：匹配
          required: false
          example: '0'
          schema:
            type: string
        - name: regex
          in: query
          description: 0：不匹配正则，非0：匹配
          required: false
          example: '0'
          schema:
            type: string
        - name: diacritics
          in: query
          description: 0：变音标记，非0：匹配
          required: false
          example: '0'
          schema:
            type: string
        - name: path_column
          in: query
          description: 非0时返回结果包含路径
          required: false
          example: '1'
          schema:
            type: string
        - name: size_column
          in: query
          description: 非0时返回结果包含大小
          required: false
          example: '1'
          schema:
            type: string
        - name: date_modified_column
          in: query
          description: 非0时返回结果包含修改日期
          required: false
          example: '1'
          schema:
            type: string
        - name: date_created_column
          in: query
          description: 非0时返回结果包含创建日期
          required: false
          example: '1'
          schema:
            type: string
        - name: attributes_column
          in: query
          description: 非0时返回结果包含结果属性
          required: false
          example: '1'
          schema:
            type: string
        - name: ascending
          in: query
          description: 0：降序，非0：升序
          required: false
          example: '0'
          schema:
            type: string
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: object
                properties:
                  totalResults:
                    type: integer
                  results:
                    type: array
                    items:
                      type: object
                      properties:
                        type:
                          type: string
                        name:
                          type: string
                        path:
                          type: string
                        size:
                          type: string
                        date_modified:
                          type: string
                      required:
                        - type
                        - name
                        - path
                        - size
                        - date_modified
                required:
                  - totalResults
                  - results
              example:
                totalResults: 31
                results:
                  - type: file
                    name: index.css
                    path: >-
                      C:\Users\caolib\AppData\Roaming\Tencent\xwechat\radium\xworker\search\resources\ilinkres_9eab8389\55\dist
                    size: '1901'
                    date_modified: '134015353195589871'
                  - type: file
                    name: index.css
                    path: >-
                      C:\software\dev\Trae\resources\app\extensions\markdown-math\preview-styles
                    size: '424'
                    date_modified: '134012985740000000'
                  - type: file
                    name: index.css
                    path: >-
                      C:\Users\caolib\AppData\Local\Microsoft\Edge\User
                      Data\Default\Extensions\inedkoakiaeepjoblbiiipedngonadhn\1.6.27_0\webfont
                    size: '1799'
                    date_modified: '134012514860000000'
                  - type: file
                    name: index.css
                    path: C:\code\Hexo\hexo_blog_src\public\css
                    size: '684493'
                    date_modified: '134007411964021580'
                  - type: file
                    name: index.css
                    path: C:\code\Hexo\hexo_blog_src\.deploy_git\css
                    size: '684493'
                    date_modified: '134007411964021580'
                  - type: file
                    name: index.css
                    path: >-
                      C:\$Recycle.Bin\S-1-5-21-1222158067-506126031-1967164690-1001\$RYL2Q1Q\node_modules\.pnpm\element-ui@2.15.14_vue@2.7.16\node_modules\element-ui\packages\theme-chalk\lib
                    size: '240033'
                    date_modified: '134003883189081899'
                  - type: file
                    name: index.css
                    path: >-
                      C:\$Recycle.Bin\S-1-5-21-1222158067-506126031-1967164690-1001\$RYL2Q1Q\node_modules\.pnpm\element-ui@2.15.14_vue@2.7.16\node_modules\element-ui\lib\theme-chalk
                    size: '240033'
                    date_modified: '134003883189081899'
                  - type: file
                    name: index.css
                    path: >-
                      C:\$Recycle.Bin\S-1-5-21-1222158067-506126031-1967164690-1001\$RYL2Q1Q\node_modules\.pnpm\vue-area-linkage@5.1.0_area-data@5.0.6_vue@2.7.16\node_modules\vue-area-linkage\dist
                    size: '4310'
                    date_modified: '134003883177314800'
                  - type: file
                    name: index.css
                    path: >-
                      C:\software\dev\Microsoft VS
                      Code\resources\app\extensions\markdown-math\preview-styles
                    size: '416'
                    date_modified: '134001542780000000'
                  - type: file
                    name: index.css
                    path: C:\code\markdown\bin_md_notes\docs\.vitepress\theme\styles
                    size: '358'
                    date_modified: '134001330903581111'
                  - type: file
                    name: index.css
                    path: >-
                      C:\code\Vue\vue3-vite\node_modules\element-plus\theme-chalk
                    size: '326636'
                    date_modified: '133999860185444205'
                  - type: file
                    name: index.css
                    path: C:\code\Vue\vue3-vite\node_modules\element-plus\dist
                    size: '326636'
                    date_modified: '133999860183522736'
                  - type: file
                    name: index.css
                    path: >-
                      C:\$Recycle.Bin\S-1-5-21-1222158067-506126031-1967164690-1001\$R9RWWKE\node_modules\.pnpm\element-plus@2.4.3_vue@3.4.0\node_modules\element-plus\theme-chalk
                    size: '326636'
                    date_modified: '133999591576041850'
                  - type: file
                    name: index.css
                    path: >-
                      C:\$Recycle.Bin\S-1-5-21-1222158067-506126031-1967164690-1001\$R9RWWKE\node_modules\.pnpm\element-plus@2.4.3_vue@3.4.0\node_modules\element-plus\dist
                    size: '326636'
                    date_modified: '133999591576041850'
                  - type: file
                    name: index.css
                    path: >-
                      C:\$Recycle.Bin\S-1-5-21-1222158067-506126031-1967164690-1001\$R9RWWKE\node_modules\.pnpm\vue-cropper@1.1.1\node_modules\vue-cropper\dist
                    size: '3197'
                    date_modified: '133999591483903568'
                  - type: file
                    name: index.css
                    path: >-
                      C:\$Recycle.Bin\S-1-5-21-1222158067-506126031-1967164690-1001\$RU8JFOL\ruoyi-ui\public\styles\theme-chalk
                    size: '239846'
                    date_modified: '133999549322867622'
                  - type: file
                    name: index.css
                    path: >-
                      C:\code\Tauri\tauri-template\node_modules\.pnpm\@simonwep+pickr@1.8.2\node_modules\@simonwep\pickr\www
                    size: '2602'
                    date_modified: '133996077589211632'
                  - type: file
                    name: index.css
                    path: >-
                      C:\code\Tauri\wem\node_modules\.pnpm\@element-plus+theme-chalk@2.2.16\node_modules\@element-plus\theme-chalk\dist
                    size: '322734'
                    date_modified: '133986888868062587'
                  - type: file
                    name: index.css
                    path: >-
                      C:\code\Tauri\wem\node_modules\.pnpm\element-plus@2.10.5_vue@3.5.18\node_modules\element-plus\theme-chalk
                    size: '337104'
                    date_modified: '133986677248153253'
                  - type: file
                    name: index.css
                    path: >-
                      C:\code\Tauri\wem\node_modules\.pnpm\element-plus@2.10.5_vue@3.5.18\node_modules\element-plus\dist
                    size: '337104'
                    date_modified: '133986677248153253'
          headers: {}
          x-apifox-name: 成功
      security: []
      x-apifox-folder: ''
      x-apifox-status: developing
      x-run-in-apifox: https://app.apifox.com/web/project/7080207/apis/api-346992499-run
components:
  schemas: {}
  securitySchemes: {}
servers:
  - url: http://localhost:8080
    description: 本地8080
security: []

```