let activeIndex = 0;

const projects = document.getElementsByClassName('project');
const page_no = document.getElementById('page_no');

const handleLeftClick = () => {
    projects[activeIndex].classList.remove('active');
    projects[activeIndex].classList.add('inactive');
    activeIndex = (activeIndex + projects.length - 1 ) % projects.length;
    projects[activeIndex].classList.remove('inactive');
    projects[activeIndex].classList.add('active');

    console.log(activeIndex);
    console.log(page_no);
    page_no.innerText = activeIndex + " / " + projects.length;
}

const handleRightClick = () => {
    projects[activeIndex].classList.remove('active');
    projects[activeIndex].classList.add('inactive');
    activeIndex = (activeIndex + 1 ) % projects.length;
    projects[activeIndex].classList.remove('inactive');
    projects[activeIndex].classList.add('active');

    page_no.innerText = activeIndex + " / " + projects.length;
}
