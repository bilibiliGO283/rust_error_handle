# git ��������
* git branch [branch]
* git checkout [HEAD]|[branch]
* git checkout --rebase [branch]
* git rebase [branch]
    - �ѵ�ǰ�ύ  �ƶ��� branch
* git reset
* git revert
* git push
* git pull
* git cherry-pick [branch]
* git rebase -i [HEAD] 
* git checkout -b [branch] [o/branch] 
    - �ڱ��ش���һ����֧����Զ�̵�һ����֧
* git branch -u o/master [branch] 
    - ����ǵ�ǰ��֧�Ϳ���ʡ��
* git push origin foo 
    - �ѱ���foo��֧����û��push��  push��Զ�ֿ̲�
* git push origin master:newBranch 
    - �ѱ���master��֧����û�����͵ģ����͵�Զ��newBrand��֧��
* git fetch origin foo 
    - ��Զ�̵�foo��֧������û�����ص��ύ���ص�����
* git fetch origin foo~1:newBranch 
    - ��foo~1 ����һ��newBranch��֧������
* git push origin :master 
    - ��ɾ��Զ��master��֧
* git fetch orgin :foo 
    - ���ڵ�ǰ�ڵ㴴��һ���·�֧
* git pull origin master:foo 
    - ��Զ��master��֧���ύ��������������foo��֧�����Һϲ���ǰ��֧��master�ϣ�
* git branch -u origin/master foo 
    - ��ط�֧foo����Զ�̷�֧master
* git checkout -b foo origin/master 
    - ����һ��foo��֧����Զ�̷�֧master