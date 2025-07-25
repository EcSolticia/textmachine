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

function strip_end(content, endw)
  local nc = #content
  local ne = #endw

  if ne <= nc and string.sub(content, -ne) == endw then
    return string.sub(content, 1, nc - ne)
  else
    return content
  end
end

function md_to_html_link(link_target)
  local tm1 = strip_end(link_target, ".md")
  local tm2 = strip_end(link_target, ".md/")

  local tm = ""

  if tm1 ~= link_target then tm = tm1
  elseif tm2 ~= link_target then tm = tm2
  else return link_target end

  return tm .. ".html"
end

function Link(el)
  local new_target = md_to_html_link(el.target)
  return pandoc.Link(el.content, new_target, el.title)
end
