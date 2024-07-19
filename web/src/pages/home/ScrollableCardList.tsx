import React, { useRef, useEffect, useState } from 'react';  
import { Avatar, Card, List } from '@arco-design/web-react';
  
const ScrollableCard = (props:{content}) => {  
  const scrollContainerRef = useRef(null);  
  
  
  return (  
          <List
            listRef={scrollContainerRef}
            // className={styles['scrollbar-hidden']}      
            style={{minHeight:'485px', maxHeight:'485px'}}    
            dataSource={props.content}
            grid={{ gutter: 0, span: 6 }}
            render={(item, index) => (
                <List.Item.Meta
                  key={index}
                  avatar={<Avatar shape="square" size={64} >
                    <img src={item.icon} />
                  </Avatar>}
                  title={item.prize_name}
                  description={'X'+item.remaining_quantity}
                />
              )}
          />
  );  
};  
  
export default ScrollableCard;