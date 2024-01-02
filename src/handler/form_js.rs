pub static JS_CODE: &str = r#"
	(function () {
	    const array = [];
	    for (let index = 0; index < document.forms.length; index++) {
	        const form = document.forms[index];
		    const labels = document.querySelectorAll('label');
		    const labels_length = labels.length;
    	    for (let i = 0; i < form.elements.length; i++) {
    	        const ee = form.elements[i];
		    	const tagName = ee.tagName.toUpperCase();
    	        if ("INPUT" === tagName || "SELECT" === tagName || "TEXTAREA" === tagName ) {
		    		let readonly = ee.getAttribute('readonly');
    	            array.push({ 'tag_name': ee.tagName, 'name': ee.name, 'el_type': ee.type, 'id': ee.id, 'class_name': ee.className,
		    				'xpath': getPathTo(ee), 'readonly': readonly === null ? "" : readonly, 'label': labels_length > i ? labels[i].innerText: ""
    	            })
    	        } else if ("BUTTON" === tagName) {
		    		array.push({ 'tag_name': ee.tagName, 'name': ee.name, 'el_type': ee.type, 'id': ee.id, 'class_name': ee.className,
		    				'xpath': getPathTo(ee), 'readonly': "", 'label': ""
    	            })
		    	}
    	    }
	    }
    	return array;
    })();

    function getPathTo(element) {
        if (element.id !== '')
			return `/html/id("${element.id}")`;
        if (element === document.body) 
            return `/html/${element.tagName.toLowerCase()}`;
        const index = [...element.parentNode.children].filter((child) => child.tagName === element.tagName).indexOf(element) + 1;
        const tagName = element.tagName.toLowerCase();
        const parentPath = getPathTo(element.parentNode);
        return `${parentPath}/${tagName}[${index}]`;
    }
"#;
