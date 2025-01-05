let activeIndex = 0;

const projects = document.getElementsByClassName('project');

const handleLeftClick = () => {
    projects[activeIndex].classList.remove('active');
    projects[activeIndex].classList.add('inactive');
    activeIndex = (activeIndex + projects.length - 1 ) % projects.length;
    projects[activeIndex].classList.remove('inactive');
    projects[activeIndex].classList.add('active');
}

const handleRightClick = () => {
    projects[activeIndex].classList.remove('active');
    projects[activeIndex].classList.add('inactive');
    activeIndex = (activeIndex + 1 ) % projects.length;
    projects[activeIndex].classList.remove('inactive');
    projects[activeIndex].classList.add('active');
}
