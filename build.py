import os
import subprocess

def create_if_not_exists(dir):
  if not os.path.exists(dir):
      os.makedirs(dir)

md_dir = './Recipes'
intermediate_dir = 'intermediate'
intermediate_recipe = os.path.join(intermediate_dir,'recipes')
intermediate_tags = os.path.join(intermediate_dir,'tags')
out_dir = 'html'
out_recipes = os.path.join(out_dir,'recipes')
out_tags = os.path.join(out_dir,'tags')
pandoc_sh = './build.sh'
index_template = 'index.html'
index_path = os.path.join(out_dir,index_template)

print('ensuring intermediate directories exist')
create_if_not_exists(intermediate_dir)
create_if_not_exists(intermediate_recipe)
create_if_not_exists(intermediate_tags)

tag_dict = {} 
recipe_dict = {}
print('preprocessing recipe markdown files')

files = os.listdir(md_dir)
files.sort()
for fn in files:
    print('preprocessing %s' % fn)
    full_name = os.path.join(md_dir,fn)
    site_name = os.path.splitext(os.path.basename(full_name))[0]
    recipe_name = site_name

    fl = open(full_name,'r')
    contents = fl.read()
    lines = contents.split('\n')

    tagsLine = -1 
    for lineNr in range(len(lines)):
        if lines[lineNr].startswith('# '): 
            recipe_name = lines[lineNr].replace('# ','')
            lines[lineNr] = '% '+recipe_name 
        if lines[lineNr].startswith('#### Tags'):
            tagsLine = lineNr+1
            break
    
    tags = lines[tagsLine].split(', ')
    recipe_dict[site_name] = recipe_name 
    for tag in tags:
        if tag not in tag_dict.keys():
            tag_dict[tag] = { 'recipes': [site_name] , 'tag_name':tag.capitalize()}
        else:
            tag_dict[tag]['recipes'].append(site_name)

        lines[tagsLine] = lines[tagsLine].replace(tag,'[%s](../tags/%s.html)' % (tag_dict[tag]['tag_name'],tag))

    lines.append('')
    lines.append('[back](../index.html)')
    new_name = os.path.join(intermediate_recipe,fn)
    fl = open(new_name,'w')
    contents = '\n'.join(lines)
    fl.write(contents)
    fl.close()

print('creating markdown files for tags')
tags = list(tag_dict.keys())
tags.sort()
for tag in tags:
    print('creating markdown for %s' % tag_dict[tag]['tag_name'])
    md_text = '%% %s\n\n' % tag_dict[tag]['tag_name']
    for recipe_name in tag_dict[tag]['recipes']:
        md_text += '* [%s](../recipes/%s.html) \n' % (recipe_dict[recipe_name],recipe_name)
    md_text += '\n\n[back](../index.html)'
    tag_path = os.path.join(intermediate_tags,'%s.md' % tag)
    tag_file = open(tag_path,'w')
    tag_file.write(md_text)
    tag_file.close()

print('ensuring html dirs exist')
create_if_not_exists(out_dir)
create_if_not_exists(out_recipes)
create_if_not_exists(out_tags)

print('running pandoc build')
subprocess.call(pandoc_sh)

print('creating index file')

print('creating recipe list')
recipes_str = ''
for recipe_site in recipe_dict.keys():
    recipe_str = '<li><a href="recipes/%s.html">%s</a></li>' % (recipe_site, recipe_dict[recipe_site])
    recipes_str += recipe_str + '\n'

print('creating tag list')
tags_str = ''
for tag_name in tag_dict.keys():
    tag_str = '<li><a href="tags/%s.html">%s</a></li>' % (tag_name,tag_dict[tag_name]['tag_name'])
    tags_str += tag_str + '\n'

print('inserting html in template')
templ_file = open(index_template,'r')
templ_contents = templ_file.read()
templ_contents = templ_contents.replace('{{- recipes -}}', recipes_str)
templ_contents = templ_contents.replace('{{- tags -}}',tags_str)

index_file = open(index_path,'w')
index_file.write(templ_contents)
