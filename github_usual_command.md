# git 常用命令
* git branch [branch]
* git checkout [HEAD]|[branch]
* git checkout --rebase [branch]
* git rebase [branch]
    - 把当前提交  移动到 branch
* git reset
* git revert
* git push
* git pull
* git cherry-pick [branch]
* git rebase -i [HEAD] 
* git checkout -b [branch] [o/branch] 
    - 在本地创建一个分支跟踪远程的一个分支
* git branch -u o/master [branch] 
    - 如果是当前分支就可以省略
* git push origin foo 
    - 把本地foo分支所有没有push的  push到远程仓库
* git push origin master:newBranch 
    - 把本地master分支所有没有推送的，推送到远程newBrand分支上
* git fetch origin foo 
    - 把远程的foo分支的所有没有下载的提交下载到本地
* git fetch origin foo~1:newBranch 
    - 从foo~1 创建一个newBranch分支到本地
* git push origin :master 
    - 会删除远程master分支
* git fetch orgin :foo 
    - 会在当前节点创建一个新分支
* git pull origin master:foo 
    - 把远程master分支的提交下载下来，生成foo分支，并且合并当前分支到master上，
* git branch -u origin/master foo 
    - 令本地分支foo跟踪远程分支master
* git checkout -b foo origin/master 
    - 创建一个foo分支跟踪远程分支master