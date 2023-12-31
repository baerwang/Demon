pub static JS_CODE: &str = r#"
data()

	function data() {
	    let array = [];
	    for (let index = 0; index < document.forms.length; index++) {
	        let form = document.forms[index];
		    let labels = document.querySelectorAll('label');
		    let labels_length = labels.length;
    	    for (let i = 0; i < form.elements.length; i++) {
    	        let ee = form.elements[i];
		    	let tagName = ee.tagName.toUpperCase();
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
    }

	function getPathTo(element) {
		if (element.id !== '')
			return 'id("' + element.id + '")';
		if (element === document.body)
			return element.tagName;
		let index = 0;
		let siblings = element.parentNode.childNodes;
		for (let i = 0; i < siblings.length; i++) {
			let sibling = siblings[i];
			if (sibling === element)
				return getPathTo(element.parentNode) + '/' + element.tagName + '[' + (index + 1) + ']';
			if (sibling.nodeType === 1 && sibling.tagName === element.tagName)
				index++;
		}
	}
"#;
