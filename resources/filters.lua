function Header (h)
  if h.identifier ~= '' then
    local anchor_link = pandoc.Link(
      "(<)",
      '#' .. h.identifier,
      '',
      {class = 'anchor'}
    )
    table.insert(h.content, anchor_link)
    return h
  end
end

function link_md_to_html(target)
  return target:gsub("%.md([#?])", ".html%1")
               :gsub("%.md$", ".html")
end

function Link(el)
  local new_target = link_md_to_html(el.target)
  
  return pandoc.Link(el.content, new_target, el.title)
end
