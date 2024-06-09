import os
import subprocess
import jinja2
import datetime

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
footer_template_name = 'footer.html'
tag_overview_template_name = 'tag_overview.html'
    
md_dir = './Recipes'
intermediate_dir    = os.path.join('intermediate','md')
intermediate_recipe = os.path.join(intermediate_dir,'recipes')
    
pandoc_sh       = './build.sh'
pandoc_dir      = os.path.join('intermediate','html')
pandoc_recipes  = os.path.join(pandoc_dir,'recipes')
pandoc_tags     = os.path.join(pandoc_dir,'tags')
    
out_dir     = 'html'
out_recipes = os.path.join(out_dir,'recipes')
out_tags    = os.path.join(out_dir,'tags')
images_dir  = 'img'
    
needed_dirs = [
        intermediate_dir,intermediate_recipe,
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
        self.footer_template = env.get_template(footer_template_name)
        self.tag_overview_template = env.get_template(tag_overview_template_name)
    
    def render_index(self) :
        recipes_str = self.create_recipe_list()
        header_str = self.header_template.render(index_link='index.html',tag_link='tag_overview.html')
        created_date = datetime.datetime.now().strftime('%d.%m.%Y')
        num_recipes = len(self.recipe_dict.keys())
        footer_str = self.footer_template.render(created_date=created_date,modified_date='Number of recipes: '+str(num_recipes))
        index_html = self.index_template.render(recipes=recipes_str,header=header_str,footer=footer_str)
        write_file(out_dir,'index.html',index_html)

    def render_tag_overview(self):
        tags_str = self.create_tag_list()
        header_str = self.header_template.render(index_link='index.html',tag_link='tag_overview.html')
        created_date = datetime.datetime.now().strftime('%d.%m.%Y')
        num_recipes = len(self.recipe_dict.keys())
        footer_str = self.footer_template.render(created_date=created_date,modified_date='Number of recipes: '+str(num_recipes))
        overview_html = self.tag_overview_template.render(tags=tags_str,header=header_str,footer=footer_str,)
        write_file(out_dir,'tag_overview.html',overview_html)

    def render_recipe_template(self,recipe_file_name):
        recipe_base = os.path.splitext(recipe_file_name)[0]
        html_title = self.recipe_dict[recipe_base]['recipe_name']
        html_content = read_file(pandoc_recipes,recipe_file_name)
        created_date = self.recipe_dict[recipe_base]['created_date'].strftime('%d.%m.%Y %H:%M')
        modified_date = self.recipe_dict[recipe_base]['modified_date'].strftime('%d.%m.%Y %H:%M')
        footer_str = self.footer_template.render(created_date=created_date,modified_date='Last modified: '+modified_date)
        header_str = self.header_template.render(index_link='../index.html',tag_link='../tag_overview.html')
        recipe_images = []
        images_path = os.path.join(out_dir,images_dir)
        recipe_images_files = os.listdir(images_path)
        recipe_images_files.sort()
        recipe_images_files = list(filter(lambda x: recipe_base in x,recipe_images_files))
        recipe_image_template = '<div class="recipe_image"><img src="%s"/></div>'
        for recipe_image_file in recipe_images_files:
            recipe_image_path = os.path.join('..',images_dir,recipe_image_file)
            recipe_images.append(recipe_image_template % recipe_image_path)
        recipe_images_str = '<div id="recipe_images">%s</div>' %'\n'.join(recipe_images)
        recipe_dict  = {
                'recipe_images':recipe_images_str,
                'content':html_content,
                'header':header_str,
                'title':html_title,
                'footer':footer_str}
        curr_html = self.recipe_template.render(recipe_dict)
        write_file(out_recipes,recipe_file_name,curr_html)

    def render_tag_template(self,tag_file_name):
        tag_base = os.path.splitext(tag_file_name)[0]
        html_title = self.tag_dict[tag_base]['tag_name']
        html_content = read_file(pandoc_tags,tag_file_name)
        created_date = datetime.datetime.now().strftime('%d.%m.%Y')
        footer_str = self.footer_template.render(created_date=created_date,modified_date='')
        header_str = self.header_template.render(index_link='../index.html',tag_link='../tag_overview.html')
        curr_html = self.tag_template.render(content=html_content,header=header_str,title=html_title,footer=footer_str)
        write_file(out_tags,tag_file_name,curr_html)

    def render_recipe_details(self):
        in_dir = pandoc_recipes 
        file_ls = os.listdir(in_dir)
        file_ls.sort()
        for file_name in file_ls:
            self.render_recipe_template(file_name)

  
    def process_file(self,src_name):
        print('preprocessing %s' % src_name)
        src_contents = read_file(md_dir,src_name)
        file_path = os.path.join(md_dir,src_name)
        created_date = datetime.datetime.fromtimestamp(os.path.getctime(file_path))
        modified_date = datetime.datetime.fromtimestamp(os.path.getmtime(file_path))

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

            if curr_line.startswith('## Tags'):
                (recipe_tags,tag_line) = self.update_tagline(src_lines[line_nr+1],src_name_base)
                tags = recipe_tags
                new_lines.append(curr_line)
                new_lines.append(tag_line)
                skip = line_nr + 1
                continue

            new_lines.append(curr_line)
        
        recipe_information = { 'recipe_name':recipe_name, 'tags':tags, 'created_date':created_date, 'modified_date':modified_date }
        self.recipe_dict[src_name_base] = recipe_information
        src_contents = '\n'.join(new_lines)
        write_file(intermediate_recipe,src_name,src_contents)
    
    def update_tagline(self,line,recipe): 
        tags = line.split(',')
        if len(tags) == 0:
            print('No tags for %s' % recipe)
        tags = list(map(lambda x: x.strip(),tags))
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
    
    def render_tag_details(self,tag):
        tag_name : str = self.tag_dict[tag]['tag_name']

        tag_dict = {}
        tag_dict['title']  = tag_name
        tag_dict['header'] = self.header_template.render(index_link='../index.html',tag_link='../tag_overview.html')
        created_date : str = datetime.datetime.now().strftime('%d.%m.%Y')
        num_recipes  : int = len(self.recipe_dict.keys())
        tag_dict['footer'] = self.footer_template.render(created_date=created_date,modified_date='Number of recipes: '+str(num_recipes))
        
        content_strs : list[str] = []
        recipe_li_template = '<div class="recipe_item"><a href="../recipes/%s.html">%s</a></div>'
        for recipe in self.tag_dict[tag]['recipes']:
            recipe_str = recipe_li_template % (self.recipe_dict[recipe]['recipe_name'],recipe)
            content_strs.append(recipe_str)

        tag_dict['content'] = '\n'.join(content_strs)
        tag_html = self.tag_template.render(tag_dict)
        write_file(out_tags,tag+'.html',tag_html)

        
    def create_tag_list(self):
        li_template = '<div class="tag_item"><a href="tags/%s.html">%s</a>&nbsp;(%s)</div>\n'
        tags_str = ''
        for tag in self.tag_dict.keys():
            tag_nr = len(self.tag_dict[tag]['recipes'])
            tags_str += li_template % (tag,self.tag_dict[tag]['tag_name'],str(tag_nr))
        return tags_str


    def create_recipe_list(self):
        li_start = '<div class="recipe_item"><a href="recipes/%s.html">%s</a>'
        li_end = '</div>\n'
        recipes_str = ''

        current_char : str = chr(ord('A')-1)
        letter_separator : str = '<div class="recipe_letter">%s</div>'

        for recipe_base in self.recipe_dict.keys():
            if current_char != recipe_base[0].upper():
                current_char = chr(ord(current_char)+1)
                recipes_str += letter_separator % current_char 
            recipes_str += li_start % (recipe_base,self.recipe_dict[recipe_base]['recipe_name'])
            tags = self.recipe_dict[recipe_base]['tags']

            if not tags == []:
                recipes_str += '<div class="recipe_taglist">Tags:'
                for tag in tags: 
                    recipes_str += '<a href="tags/%s.html">%s</a>,&nbsp;'%(tag,tag)
                recipes_str += '</div>'
            
            created_date = str(self.recipe_dict[recipe_base]['created_date'].timestamp())
            modified_date = str(self.recipe_dict[recipe_base]['modified_date'].timestamp())
            recipes_str += '<div class="metadata_created">%s</div>' % created_date
            recipes_str += '<div class="metadata_modified">%s</div>' % modified_date

            recipes_str += li_end
        return recipes_str
    
    
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
            self.render_tag_details(tag)
        
        print('running pandoc build')
        subprocess.call(pandoc_sh)
        
        print('rendering recipe pages') 
        self.render_recipe_details()

        print('creating tag overview')
        self.render_tag_overview()
         
        print('creating index file')
        self.render_index() 

if __name__ == '__main__':
    runner = HTMLBuilder()
    runner.run_build()
else: 
    print(__name__)
