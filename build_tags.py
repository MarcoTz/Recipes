import os 

md_dir = './Recipes'
intermediate_dir = './intermediate'

tagDict = {} 
print('adding links to tags')
for fn in os.listdir(md_dir):
    full_name = os.path.join(md_dir,fn)
    site_name = os.path.splitext(os.path.basename(full_name))[0]
    fl = open(full_name,'r')
    contents = fl.read()
    lines = contents.split('\n')

    tagsLine = -1 
    for lineNr in range(len(lines)):
        if lines[lineNr].startswith('#### Tags'):
            tagsLine = lineNr+1
            break
    
    tags = lines[tagsLine].split(', ')
    for tag in tags:
        if tag not in tagDict.keys():
            tagDict[tag] = [site_name]
        else:
            tagDict[tag].append(site_name)

        lines[tagsLine] = lines[tagsLine].replace(tag,'[%s](../tags/tag_%s.html)' % (tag,tag))

    lines.append('')
    lines.append('[back](../index.html)')

    new_name = os.path.join(intermediate_dir,fn)
    fl = open(new_name,'w')
    contents = '\n'.join(lines)
    fl.write(contents)

print('creating markdown files for tags')
for tag in tagDict.keys():
    md_text = '%% %s\n\n' % tag
    for recipe in tagDict[tag]:
        md_text += '* [%s](../recipes/%s.html) \n' % (recipe,recipe)
    md_text += '\n\n[back](../index.html)'
    tag_path = os.path.join(intermediate_dir,'tag_%s.md' % tag)
    tag_file = open(tag_path,'w')
    tag_file.write(md_text)
