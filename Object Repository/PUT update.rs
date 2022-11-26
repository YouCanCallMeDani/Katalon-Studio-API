<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT Update</name>
   <tag></tag>
   <elementGuidId>a4675b66-90f3-4d4f-9d98-7e7d1cb9595f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;${name}\&quot;: \&quot;${namevalue}\&quot;,\n    \&quot;${job}\&quot;: \&quot;${jobvalue}\&quot;\n}&quot;,
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
      <webElementGuid>1c58a3e2-9460-4a00-b670-0827718b3165</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${BaseURL}${PUTUpdate}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BaseURL</defaultValue>
      <description></description>
      <id>8ca40c0a-9956-4f4d-aa9d-f0741e1b62e9</id>
      <masked>false</masked>
      <name>BaseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PUTUpdate</defaultValue>
      <description></description>
      <id>29252dd5-83f3-451c-a034-40b00b8d0d56</id>
      <masked>false</masked>
      <name>PUTUpdate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.name</defaultValue>
      <description></description>
      <id>a9b9810d-f297-4550-9b56-3213b6dab833</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.namevalue</defaultValue>
      <description></description>
      <id>11bb8f28-397e-42c6-b150-092296ecbfd9</id>
      <masked>false</masked>
      <name>namevalue</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.job</defaultValue>
      <description></description>
      <id>ef24ed98-d3ea-43cf-9647-2fd350caec80</id>
      <masked>false</masked>
      <name>job</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.jobvalue</defaultValue>
      <description></description>
      <id>d320bab7-5ec3-44f7-8550-9d091b62f341</id>
      <masked>false</masked>
      <name>jobvalue</name>
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
