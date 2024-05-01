function filter(){
  if(document.body.id == 'body_index') { filter_recipes(); }
  if(document.body.id == 'body_tag_overview') { filter_tags(); }
}

function sort(){
  if(document.body.id == 'body_index') { sort_recipes(); }
  if(document.body.id == 'body_tag_overview') { sort_tags(); }
}

function sort_tags() {
  tags_ul = document.getElementById('tags_list');
  sort_select = document.getElementById('sort_key');

  sort_key = sort_select.value

  sort_fun_asc = function(li1,li2) { return (-1)*(li1.textContent.localeCompare(li2.textContent));};
  sort_fun_desc = function(li1,li2) { return (li1.textContent.localeCompare(li2.textContent));};

  if( sort_key == 'name_asc') { sort_ul(tags_ul,sort_fun_asc);}
  else { sort_ul(tags_ul,sort_fun_desc);}

}

function sort_recipes() {
  recipes_ul = document.getElementById('recipe_list');
  sort_select = document.getElementById('sort_key');

  sort_key = sort_select.value;
  
  sort_fun_asc = function(li1,li2) { return (-1)*(li1.textContent.localeCompare(li2.textContent)); };
  sort_fun_desc = function(li1,li2) { return (li1.textContent.localeCompare(li2.textContent)); };

  if(sort_key == 'name_asc') { sort_ul(recipes_ul,sort_fun_asc); }
  else { sort_ul(recipes_ul,sort_fun_desc); }

}

function sort_ul(ul,sort_fun){
  var new_ul = ul.cloneNode(false);

  var lis = [];
  for(var i=0;i<ul.children.length;i++){ lis.push(ul.children[i]); }
  
  lis.sort(sort_fun);

  for(var i=0;i<lis.length;i++){ new_ul.appendChild(lis[i]); }

  ul.parentNode.replaceChild(new_ul,ul);
}


function filter_general(html_elements,filter_fun) {
  search_value = document.getElementById('search').value.toLowerCase();
  for(element of html_elements){
    if(filter_fun(element,search_value)) { element.style.display='block';}
    else { element.style.display='none'; }
  }
}

function filter_recipes() {
  recipes = document.getElementById('recipe_list').children;

  filter_fun = function(recipe_element,search_value) {
    recipe_name = recipe_element.textContent.toLowerCase();
    recipe_tags = ''
    for(child of recipe_element.children) {
      if(child.className == 'recipe_taglist') { recipe_tags = child.textContent; break; }
    }
    matches_name = recipe_name.includes(search_value);
    matches_tags = recipe_tags.includes(search_value);
    return matches_name || matches_tags;
  };

  filter_general(recipes,filter_fun);
 
}

function filter_tags() {
  tags = document.getElementById('tags_list').children;

  filter_fun = function(tagElement,search_value) {
    tag_text = tagElement.textContent.toLowerCase();
    return tag_text.includes(search_value);
  }

  filter_general(tags,filter_fun);
}
