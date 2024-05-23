<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit Highlight Unsuccess not inpit en field</name>
   <tag></tag>
   <elementGuidId>5f0a1b42-51fe-476e-9992-9551c73e0254</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;id\&quot;: \&quot;${id_highlight}\&quot;,\n   \&quot;title\&quot;:{\n      \&quot;th\&quot;:\&quot;test edit test\&quot;,\n      \&quot;en\&quot;:\&quot;\&quot;\n   },\n   \&quot;description\&quot;:{\n      \&quot;th\&quot;:\&quot;qwewq\&quot;,\n      \&quot;en\&quot;:\&quot;\&quot;\n   },\n   \&quot;content\&quot;:{\n      \&quot;th\&quot;:\&quot;wqewq\&quot;,\n      \&quot;en\&quot;:\&quot;\&quot;\n   },\n   \&quot;sender\&quot;:{\n      \&quot;th\&quot;:\&quot;qweqwewq\&quot;,\n      \&quot;en\&quot;:\&quot;\&quot;\n   },\n   \&quot;sentAt\&quot;:null,\n   \&quot;buttonValue\&quot;:\&quot;08515180658\&quot;,\n   \&quot;buttonType\&quot;:\&quot;call\&quot;,\n   \&quot;buttonName\&quot;:{\n      \&quot;th\&quot;:\&quot;Button\&quot;,\n      \&quot;en\&quot;:\&quot;\&quot;\n   },\n   \&quot;project\&quot;:{\n      \&quot;id\&quot;:\&quot;1add0c47-08fb-4ed1-b7fb-b77e61e83d21\&quot;,\n      \&quot;name\&quot;:{\n         \&quot;th\&quot;:\&quot;นิรติ บางนา\&quot;,\n         \&quot;en\&quot;:\&quot;NIRATI BANGNA\&quot;\n      }\n   },\n   \&quot;unit\&quot;:{\n      \&quot;id\&quot;:\&quot;eceff06a-2b7b-4308-abda-ca6c281e5594\&quot;,\n      \&quot;houseNumber\&quot;:\&quot;333/1\&quot;,\n      \&quot;unitNumber\&quot;:\&quot;0001\&quot;\n   }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d7b29b10-0b7f-417a-bab1-27ddc3201ea6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>462198f6-8c1c-45bf-a099-550560d29997</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/announcement-cpnr</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.id_highlight</defaultValue>
      <description></description>
      <id>4a389bd8-bf89-4e23-bd78-0d4623a624e1</id>
      <masked>false</masked>
      <name>id_highlight</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>ffdc4f86-ff71-486b-95d7-cf275a15c19a</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
