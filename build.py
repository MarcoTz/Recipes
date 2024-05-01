import os
import subprocess
import jinja2

def create_if_not_exists(dir):
    if not os.path.exists(dir):
        os.makedirs(dir)
def read_file(src_dir,src_file):
    full_name = os.path.join(src_dir,src_file)
    fl = open(full_name,'r')
    return fl.read()
    
def write_file(src_dir,src_file,file_contents):
    full_name = os.path.join(src_dir,src_file)
    fl = open(full_name,'w+')
    fl.write(file_contents)
    fl.close()

template_dir         = 'html_templates'
index_template_name  = 'index.html'
recipe_template_name = 'recipe.html'
tag_template_name    = 'tag.html'
header_template_name = 'header.html'
    
md_dir = './Recipes'
intermediate_dir    = os.path.join('intermediate','md')
intermediate_recipe = os.path.join(intermediate_dir,'recipes')
intermediate_tags   = os.path.join(intermediate_dir,'tags')
    
pandoc_sh       = './build.sh'
pandoc_dir      = os.path.join('intermediate','html')
pandoc_recipes  = os.path.join(pandoc_dir,'recipes')
pandoc_tags     = os.path.join(pandoc_dir,'tags')
    
out_dir     = 'html'
out_recipes = os.path.join(out_dir,'recipes')
out_tags    = os.path.join(out_dir,'tags')
    
needed_dirs = [
        intermediate_dir,intermediate_recipe,intermediate_tags,
        pandoc_dir,pandoc_recipes,pandoc_tags,
        out_dir,out_tags,out_recipes]

class HTMLBuilder: 
    
    
    tag_dict = {} 
    recipe_dict = {}
     
    def __init__(self):
        print('loading templates')
        env             = jinja2.Environment(loader=jinja2.FileSystemLoader(template_dir),autoescape=False)
        self.index_template  = env.get_template(index_template_name)
        self.recipe_template = env.get_template(recipe_template_name)
        self.tag_template    = env.get_template(tag_template_name)
        self.header_template = env.get_template(header_template_name)

    def render_template(self,templ,html_contents,html_title,out_file,isRecipe):
        header_str = self.header_template.render(index_link='../index.html')
        curr_html = templ.render(content=html_contents,header=header_str,title=html_title)
        out_dir = out_recipes if isRecipe else out_tags
        write_file(out_dir,out_file,curr_html)
    
    def render_index(self) :
        (recipes_str,tags_str) = self.create_html_lists()
        header_str = self.header_template.render(index_link='index.html')
        index_html = self.index_template.render(recipes=recipes_str,header=header_str,tags=tags_str)
        write_file(out_dir,'index.html',index_html)
   
    def process_file(self,src_name):
        print('preprocessing %s' % src_name)
        src_contents = read_file(md_dir,src_name)
        src_name_base = os.path.splitext(src_name)[0]
        recipe_name = ''
    
        src_lines = src_contents.split('\n')
        new_lines = []
        skip = -1   
        tags = []

        for line_nr in range(len(src_lines)):
            if line_nr == skip:
                continue

            curr_line = src_lines[line_nr]
            if curr_line.startswith('# '):
                recipe_name = curr_line.replace('# ','')
                new_lines.append(curr_line)
                continue

            if curr_line.startswith('#### Tags'):
                (recipe_tags,tag_line) = self.update_tagline(src_lines[line_nr+1],src_name_base)
                tags = recipe_tags
                new_lines.append(curr_line)
                new_lines.append(tag_line)
                skip = line_nr + 1
                continue

            new_lines.append(curr_line)
        
        recipe_information = { 'recipe_name':recipe_name, 'tags':tags }
        self.recipe_dict[src_name_base] = recipe_information
        src_contents = '\n'.join(new_lines)
        write_file(intermediate_recipe,src_name,src_contents)
    
    def update_tagline(self,line,recipe): 
        tags = line.split(', ')
        recipe_tags = []
        for tag in tags:
            tag = tag.lower()
            recipe_tags.append(tag)
            self.update_tag(tag,recipe)
            line = line.replace(tag,'[%s](../tags/%s.html)' % (tag.capitalize(),tag))
        return (recipe_tags,line)
    
    def update_tag(self,tag,recipe):
        if not tag in self.tag_dict.keys():
            self.tag_dict[tag] = { 'recipes' : [recipe], 'tag_name':tag.capitalize() }
        else:
            self.tag_dict[tag]['recipes'].append(recipe)
    
    def create_tag_markdown(self,tag):
        tag_name = self.tag_dict[tag]['tag_name']
        print('creating markdown for %s' % tag_name)
        md_text = '# %s\n\n' % tag_name 
        for recipe in self.tag_dict[tag]['recipes']:
            md_text += '* [%s](../recipes/%s.html) \n' % (self.recipe_dict[recipe]['recipe_name'],recipe)
        write_file(intermediate_tags,tag+'.md',md_text)
    
    def render_templates(self,isRecipe):
        in_dir = pandoc_recipes if isRecipe else pandoc_tags
        file_ls = os.listdir(in_dir)
        file_ls.sort()
        for file_name in file_ls:
            file_html = read_file(in_dir,file_name) 
            file_base = os.path.splitext(file_name)[0]
            file_title = self.recipe_dict[file_base]['recipe_name'] if isRecipe else self.tag_dict[file_base]['tag_name']
            templ = self.recipe_template if isRecipe else self.tag_template
            self.render_template(templ,file_html,file_title,file_name,isRecipe)
    
    def create_html_lists(self):
        li_start = '<li><a href=%s/%s.html>%s</a>'
        li_end = '</li>\n'
        recipes_str = ''
        for recipe_base in self.recipe_dict.keys():
            recipes_str += li_start % ('recipes',recipe_base,self.recipe_dict[recipe_base]['recipe_name'])
            recipes_str += '<div class="recipe_taglist">%s</div>' % ', '.join(self.recipe_dict[recipe_base]['tags'])
            recipes_str += li_end
        tags_str = ''
        for tag_base in self.tag_dict.keys():
            tags_str += li_start % ('tags',tag_base,self.tag_dict[tag_base]['tag_name'])
            tags_str += li_end
        return (recipes_str,tags_str)
    
    
    def run_build(self):
        print('Checking if all needed directories are present')
        for dir_name in needed_dirs: 
            create_if_not_exists(dir_name)
        
        
        print('preprocessing recipe markdown files')
        files = os.listdir(md_dir)
        files.sort()
        for file in files:
            self.process_file(file)
        
        print('creating markdown files for tags')
        tags = list(self.tag_dict.keys())
        tags.sort()
        for tag in tags:
            self.create_tag_markdown(tag)
        
        print('running pandoc build')
        subprocess.call(pandoc_sh)
        
        print('inserting recipe html into templates') 
        self.render_templates(True)
        print('inserting tag html into templates')
        self.render_templates(False)
        
        print('creating index file')
        self.render_index() 

if __name__ == '__main__':
    runner = HTMLBuilder()
    runner.run_build()
else: 
    print(__name__)
