import React, { useRef, useEffect, useState } from 'react';  
import { Card } from '@arco-design/web-react';
import styles from './style/index.module.less';
  
const ScrollableCard = (props:{content}) => {  
  const [scrollTop, setScrollTop] = useState(0);  
  const scrollContainerRef = useRef(null);  
  
  useEffect(() => {  
    const scrollContainer = scrollContainerRef.current;  
    if (!scrollContainer) return;  
    const totalHeight = scrollContainer.scrollHeight;  
    const visibleHeight = scrollContainer.clientHeight;  
  
    const interval = setInterval(() => {  
      const newScrollTop = (scrollTop + 1) % (totalHeight - visibleHeight);  
      setScrollTop(newScrollTop);  
      scrollContainer.scrollTop = newScrollTop;  
    }, 200);  
  
    return () => clearInterval(interval);  
  }, [scrollTop,props.content]);  
  
  return (  
    <Card title="最新动态" bordered={false} style={{ width: '100%' }}>  
      <div  
        ref={scrollContainerRef}  
        className={styles['scrollbar-hidden']}
        style={{  
          height: 150, // 设置固定高度  
        }}  
      >  
        {props.content.map((item, index) => (  
          <div key={index} style={{ padding: '10px' }}>{item}</div>  
        ))}  
      </div>  
    </Card>  
  );  
};  
  
export default ScrollableCard;