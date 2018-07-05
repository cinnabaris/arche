package web

// Pagination pagination
type Pagination struct {
	Total  int64
	Page   int64
	Size   int64
	Href   string
	Locale string
	Pages  []int64
}

// NewPagination pagination
func NewPagination(page, size, count int64) *Pagination {
	if page < 1 {
		page = 1
	}
	if size < 1 || size > 120 {
		size = 60
	}
	total := (count / size)
	if count%size > 0 {
		total++
	}
	if total < page {
		page = total
	}

	const pad = 6
	var ids []int64
	var begin int64
	var end int64
	if total <= pad*2 {
		begin = int64(1)
		end = total
	} else if page-pad <= 0 {
		begin = int64(1)
		end = pad * 2
	} else if page+pad >= total {
		begin = total - pad*2
		end = total
	} else {
		begin = page - pad
		end = page + pad
	}
	for i := begin; i <= end; i++ {
		ids = append(ids, i)
	}
	return &Pagination{
		Page:  page,
		Size:  size,
		Total: total,
		Pages: ids,
	}
}
